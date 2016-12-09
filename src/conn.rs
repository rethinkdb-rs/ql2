use std::net::TcpStream;
use std::str;
use std::io::{Write, BufRead, Read};
use std::sync::mpsc::{self, SyncSender};
use std::fmt::Debug;

use errors::*;
use ::Result;
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use bufstream::BufStream;
use protobuf::ProtobufEnum;
use scram::{ClientFirst, ServerFirst, ServerFinal};
use serde::de::Deserialize;
use serde_json::{
    Value,
    from_str, from_slice, from_value,
    to_vec,
};
use proto::{
    VersionDummy_Version as Version,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};

include!(concat!(env!("OUT_DIR"), "/conn.rs"));
include!(concat!(env!("OUT_DIR"), "/query.rs"));

const CHANNEL_SIZE: usize = 1024 * 1024;

macro_rules! error {
    ($e:expr) => {{
        let error = Error::from($e);
        Err(error)
    }}
}

/// Response value
#[derive(Debug, Clone)]
pub enum ResponseValue<T: Deserialize> {
    Write(WriteStatus),
    Read(T),
    Raw(Value),
}

/// Connection Options
///
/// Implements methods for configuring details to connect to database servers.
#[derive(Debug, Clone)]
pub struct ConnectionOpts {
    servers: Vec<&'static str>,
    db: &'static str,
    user: &'static str,
    password: &'static str,
    retries: u8,
    ssl: Option<SslCfg>,
    server: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct SslCfg {
    ca_certs: &'static str,
}

impl Default for ConnectionOpts {
    fn default() -> ConnectionOpts {
        ConnectionOpts {
            servers: vec!["localhost:28015"],
            db: "test",
            user: "admin",
            password: "",
            retries: 5,
            ssl: None,
            server: None,
        }
    }
}

/// A connection to a RethinkDB database.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    token: u64,
    broken: bool,
}

impl Connection {
    pub fn new(opts: &ConnectionOpts) -> Result<Connection> {
        let server = match opts.server {
            Some(server) => server,
            None => {
                return error!(ConnectionError::Other(String::from("No server selected.")))
            }
        };
        let mut conn = Connection {
            stream: try!(TcpStream::connect(server)),
            token: 0,
            broken: false,
        };
        let _ = try!(conn.handshake(opts));
        Ok(conn)
    }

    fn handshake(&mut self, opts: &ConnectionOpts) -> Result<()> {
        // Send desired version to the server
        let _ = try!(self.stream
                     .write_u32::<LittleEndian>(Version::V1_0 as u32));
        try!(parse_server_version(&self.stream));

        // Send client first message
        let (scram, msg) = try!(client_first(opts));
        let _ = try!(self.stream.write_all(&msg[..]));

        // Send client final message
        let (scram, msg) = try!(client_final(scram, &self.stream));
        let _ = try!(self.stream.write_all(&msg[..]));

        // Validate final server response and flush the buffer
        try!(parse_server_final(scram, &self.stream));
        let _ = try!(self.stream.flush());

        Ok(())
    }

    pub fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    pub fn set_token(&mut self, t: u64) -> &mut Self {
        self.token = t;
        self
    }

    pub fn token(&self) -> u64 {
        self.token
    }

    pub fn set_boken(&mut self, b: bool) -> &mut Self {
        self.broken = b;
        self
    }

    pub fn broken(&self) -> bool {
        self.broken
    }
}

fn parse_server_version(stream: &TcpStream) -> Result<()> {
    let resp = try!(parse_server_response(stream));
    let info: ServerInfo = try!(from_str(&resp));
    if !info.success {
        return error!(ConnectionError::Other(resp.to_string()));
    };
    Ok(())
}

fn parse_server_response(stream: &TcpStream) -> Result<String> {
    // The server will then respond with a NULL-terminated string response.
    // "SUCCESS" indicates that the connection has been accepted. Any other
    // response indicates an error, and the response string should describe
    // the error.
    let mut resp = Vec::new();
    let mut buf = BufStream::new(stream);
    let _ = try!(buf.read_until(b'\0', &mut resp));

    let _ = resp.pop();

    if resp.is_empty() {
        let msg = String::from("unable to connect for an unknown reason");
        return error!(ConnectionError::Other(msg));
    };

    let resp = try!(str::from_utf8(&resp)).to_string();
    // If it's not a JSON object it's an error
    if !resp.starts_with("{") {
        return error!(ConnectionError::Other(resp));
    };
    Ok(resp)
}

fn client_first(opts: &ConnectionOpts) -> Result<(ServerFirst, Vec<u8>)> {
    let scram = try!(ClientFirst::new(opts.user, opts.password, None));
    let (scram, client_first) = scram.client_first();

    let ar = AuthRequest {
        protocol_version: 0,
        authentication_method: String::from("SCRAM-SHA-256"),
        authentication: client_first,
    };
    let mut msg = try!(to_vec(&ar));
    msg.push(b'\0');
    Ok((scram, msg))
}

fn client_final(scram: ServerFirst, stream: &TcpStream) -> Result<(ServerFinal, Vec<u8>)> {
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = try!(from_str(&resp));

    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return error!(DriverError::Auth(err));
        } else {
            return error!(ConnectionError::Other(err));
        }
    };

    if let Some(auth) = info.authentication {
        let scram = scram.handle_server_first(&auth).unwrap();
        let (scram, client_final) = scram.client_final();
        let auth = AuthConfirmation { authentication: client_final };
        let mut msg = try!(to_vec(&auth));
        msg.push(b'\0');
        Ok((scram, msg))
    } else {
        error!(ConnectionError::Other(String::from("Server did not send authentication \
                                                            info.")))
    }
}

fn parse_server_final(scram: ServerFinal, stream: &TcpStream) -> Result<()> {
    let resp = try!(parse_server_response(stream));
    let info: AuthResponse = try!(from_str(&resp));
    if !info.success {
        let mut err = resp.to_string();
        if let Some(e) = info.error {
            err = e;
        }
        // If error code is between 10 and 20, this is an auth error
        if let Some(10...20) = info.error_code {
            return error!(DriverError::Auth(err));
        } else {
            return error!(ConnectionError::Other(err));
        }
    };
    if let Some(auth) = info.authentication {
        let _ = try!(scram.handle_server_final(&auth));
    }
    Ok(())
}

pub trait Pool {
    fn get(&self) -> Result<Connection>;
}

pub fn send<T, P>(pool: P, cfg: &ConnectionOpts, commands: String, opts: Option<String>, mut tx: SyncSender<Result<ResponseValue<T>>>) -> Result<()>
where T: 'static + Deserialize + Send + Debug,
        P: Pool,
{
    let mut query = wrap_query(QueryType::START, Some(commands), opts);
    let mut conn = pool.get()?;
    println!("{}", query);
    // Try sending the query
    {
        let mut i = 0;
        let mut write = true;
        let mut connect = false;
        while i < cfg.retries {
            // Open a new connection if necessary
            if connect {
                drop(&mut conn);
                conn = match pool.get() {
                    Ok(c) => c,
                    Err(error) => {
                        if i == cfg.retries - 1 {
                            return error!(error);
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                };
            }
            // Submit the query if necessary
            if write {
                if let Err(error) = write_query(&query, &mut conn) {
                    connect = true;
                    if i == cfg.retries - 1 {
                        return error!(error);
                    } else {
                        i += 1;
                        continue;
                    }
                }
                connect = false;
            }
            // Handle the response
            let (new_tx, tx_returned, write_opt, retry, res) = process_response::<T>(&mut query, &mut conn, tx);
            tx = new_tx;
            if let Err(error) = res {
                    write = write_opt;
                    if i == cfg.retries - 1 || !retry {
                        return error!(error);
                    }
                    if !tx_returned {
                        return error!(error);
                    } else {
                        i += 1;
                        continue;
                    }
            }
            break;
        }
    }
    Ok(())
}

fn process_response<T>(query: &mut String, conn: &mut Connection, mut tx: SyncSender<Result<ResponseValue<T>>>) -> (SyncSender<Result<ResponseValue<T>>>, bool, bool, bool, Result<()>)
    where T: 'static + Deserialize + Send + Debug
{
    let mut write = false;
    let mut retry = false;
    let (new_tx, tx_returned, new_retry, res) = handle_response::<T>(conn, tx);
    tx = new_tx;
    macro_rules! return_error {
        ($e:expr) => {{
            return (tx, tx_returned, write, retry, error!($e));
        }}
    }
    macro_rules! try {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return_error!(error),
            }
        }}
    }
    match res {
        Ok(t) => {
            match t {
                ResponseType::SUCCESS_ATOM | ResponseType::SUCCESS_SEQUENCE | ResponseType::WAIT_COMPLETE | ResponseType::SERVER_INFO | ResponseType::CLIENT_ERROR | ResponseType::COMPILE_ERROR | ResponseType::RUNTIME_ERROR  => {/* we are done */},
                ResponseType::SUCCESS_PARTIAL => {
                    *query = wrap_query(QueryType::CONTINUE, None, None);
                    if let Err(error) = write_query(query, conn) {
                        write = true;
                        retry = true;
                        return_error!(error);
                    }
                    let (new_tx, _, _, new_retry, res) = process_response::<T>(query, conn, tx);
                    tx = new_tx;
                    retry = new_retry;
                    if let Err(error) = res {
                        return_error!(error);
                    }
                },
            }
        }
        Err(error) => {
            retry = new_retry;
            match error {
                Error::Runtime(error) => {
                    match error {
                        RuntimeError::Availability(error) => {
                            match error {
                                AvailabilityError::OpFailed(msg) => {
                                    if msg.starts_with("Cannot perform write: primary replica for shard") {
                                        write = true;
                                        retry = true;
                                    }
                                    return_error!(AvailabilityError::OpFailed(msg));
                                }
                                error => return_error!(error),
                            }
                        }
                        error => return_error!(error),
                    }
                }
                error => return_error!(error),
            }
        }
    }
    (tx, tx_returned, write, retry, Ok(()))
}

fn handle_response<T>(conn: &mut Connection, mut tx: SyncSender<Result<ResponseValue<T>>>) -> (SyncSender<Result<ResponseValue<T>>>, bool, bool, Result<ResponseType>)
    where T: 'static + Deserialize + Send + Debug
{
    let (new_tx, _) = mpsc::sync_channel(CHANNEL_SIZE);
    let mut retry = false;
    macro_rules! return_error {
        ($e:expr) => {{
            return (tx, true, retry, error!($e));
        }}
    }
    macro_rules! try {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return_error!(error),
            }
        }}
    }
    macro_rules! try_tx {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(error) => return (new_tx, false, retry, error!($e)),
            }
        }}
    }
    match read_query(conn) {
        Ok(resp) => {
            let result: ReqlResponse = try!(from_slice(&resp[..]));
            let respt: ResponseType;
            if let Some(t) = ResponseType::from_i32(result.t) {
                respt = t;
            } else {
                let msg = format!("Unsupported response type ({}), returned by the database.", result.t);
                return_error!(DriverError::Other(msg));
            }
            // If the database says this response is an error convert the error 
            // message to our native one.
            let has_generic_error = match respt {
                ResponseType::CLIENT_ERROR | ResponseType::COMPILE_ERROR | ResponseType::RUNTIME_ERROR => true,
                _ => false,
            };
            let mut msg = String::new();
            if result.e.is_some() || has_generic_error {
                msg = if let Value::Array(error) = result.r.clone() {
                    if error.len() == 1 {
                        if let Some(Value::String(msg)) = error.into_iter().next() {
                            msg
                        } else {
                            return_error!(ResponseError::Db(result.r));
                        }
                    } else {
                        return_error!(ResponseError::Db(result.r));
                    }
                } else {
                    return_error!(ResponseError::Db(result.r));
                };
            }
            if let Some(e) = result.e {
                if let Some(error) = ErrorType::from_i32(e) {
                    match error {
                        ErrorType::INTERNAL => return_error!(RuntimeError::Internal(msg)),
                        ErrorType::RESOURCE_LIMIT => return_error!(RuntimeError::ResourceLimit(msg)),
                        ErrorType::QUERY_LOGIC => return_error!(RuntimeError::QueryLogic(msg)),
                        ErrorType::NON_EXISTENCE => return_error!(RuntimeError::NonExistence(msg)),
                        ErrorType::OP_FAILED => return_error!(AvailabilityError::OpFailed(msg)),
                        ErrorType::OP_INDETERMINATE => return_error!(AvailabilityError::OpIndeterminate(msg)),
                        ErrorType::USER => return_error!(RuntimeError::User(msg)),
                        ErrorType::PERMISSION_ERROR => return_error!(RuntimeError::Permission(msg)),
                    }
                } else {
                    return_error!(ResponseError::Db(result.r));
                }
            }
            if has_generic_error {
                match respt {
                    ResponseType::CLIENT_ERROR => return_error!(DriverError::Other(msg)),
                    ResponseType::COMPILE_ERROR => return_error!(Error::Compile(msg)),
                    ResponseType::RUNTIME_ERROR => return_error!(ResponseError::Db(result.r)),
                    _ => {/* not an error */},
                }
            }
            // Since this is a successful query let's process the results and send
            // them to the caller
            if let Ok(stati) = from_value::<Vec<WriteStatus>>(result.r.clone()) {
                for v in stati {
                    tx = try_tx!(tx.send(Ok(ResponseValue::Write(v))));
                }
            } else if let Ok(data) = from_value::<Vec<T>>(result.r.clone()) {
                for v in data {
                    tx = try_tx!(tx.send(Ok(ResponseValue::Read(v))));
                }
            } else {
                // Send unexpected query response
                // This is not an error according to the database
                // but the caller wasn't expecting such a response
                // so we just return it raw.
                tx = try_tx!(tx.send(Ok(ResponseValue::Raw(result.r.clone()))));
            }
            // Return response type so we know if we need to retrieve more data
            (tx, true, retry, Ok(respt))
        },
        // We failed to read the server's response so we will
        // try again as long as we haven't used up all our allowed retries.
        Err(error) => {
            retry = true;
            return_error!(error);
        },
    }
}

fn wrap_query(query_type: QueryType,
              query: Option<String>,
              options: Option<String>)
-> String {
    let mut qry = format!("[{}", query_type.value());
    if let Some(query) = query {
        qry.push_str(&format!(",{}", query));
    }
    if let Some(options) = options {
        qry.push_str(&format!(",{}", options));
    }
    qry.push_str("]");
    qry
}

fn write_query(query: &str, conn: &mut Connection) -> Result<()> {
    let query = query.as_bytes();
    let token = conn.token;
    if let Err(error) = conn.stream.write_u64::<LittleEndian>(token) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.write_u32::<LittleEndian>(query.len() as u32) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.write_all(query) {
        conn.broken = true;
        return error!(error);
    }
    if let Err(error) = conn.stream.flush() {
        conn.broken = true;
        return error!(error);
    }
    Ok(())
}

fn read_query(conn: &mut Connection) -> Result<Vec<u8>> {
    let _ = match conn.stream.read_u64::<LittleEndian>() {
        Ok(token) => token,
        Err(error) => {
            conn.broken = true;
            return error!(error);
        }
    };
    let len = match conn.stream.read_u32::<LittleEndian>() {
        Ok(len) => len,
        Err(error) => {
            conn.broken = true;
            return error!(error);
        }
    };
    let mut resp = vec![0u8; len as usize];
    if let Err(error) = conn.stream.read_exact(&mut resp) {
        conn.broken = true;
        return error!(error);
    }
    Ok(resp)
}
