use std::net::TcpStream;
use std::str;
use std::io::{Write, BufRead, Read};

use errors::*;
use ::Result;
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use bufstream::BufStream;
use scram::{ClientFirst, ServerFirst, ServerFinal};
use serde_json::{
    from_str, to_vec,
};
use proto::{
    VersionDummy_Version as Version,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};

include!(concat!(env!("OUT_DIR"), "/conn_types.rs"));

macro_rules! error {
    ($e:expr) => {{
        let error = Error::from($e);
        Err(error)
    }}
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
