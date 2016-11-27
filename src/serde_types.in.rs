use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
     success: bool,
     min_protocol_version: usize,
     max_protocol_version: usize,
     server_version: StdString,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: i32,
    authentication_method: StdString,
    authentication: StdString,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
     success: bool,
     authentication: Option<StdString>,
     error_code: Option<usize>,
     error: Option<StdString>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
     authentication: StdString,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqlResponse {
    t: i32,
    e: Option<i32>,
    r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

/// Status returned by a write command
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteStatus {
    inserted: u32,
    replaced: u32,
    unchanged: u32,
    skipped: u32,
    deleted: u32,
    errors: u32,
    first_error: Option<StdString>,
    generated_keys: Option<Vec<Uuid>>,
    warnings: Option<Vec<StdString>>,
    changes: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableOpts {
    read_mode: ReadMode,
    identifier_format: IdentifierFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "majority")]
    Majority,
    #[serde(rename = "outdated")]
    Outdated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdentifierFormat {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "uuid")]
    Uuid,
}

impl Default for TableOpts {
    fn default() -> TableOpts {
        TableOpts {
            read_mode: ReadMode::Single,
            identifier_format: IdentifierFormat::Name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangesOpts<T> {
    squash: T,
    changefeed_queue_size: u64,
    include_initial: bool,
    include_states: bool,
    include_offsets: bool,
    include_types: bool,
}

macro_rules! default_changes_opts {
    ($T:ident is $V:expr) => {
        impl Default for ChangesOpts<$T> {
            fn default() -> ChangesOpts<$T> {
                ChangesOpts {
                    squash: $V,
                    changefeed_queue_size: 100_000,
                    include_initial: true,
                    include_states: false,
                    include_offsets: false,
                    include_types: false,
                }
            }
        }
    }
}

default_changes_opts!{ bool is false }
default_changes_opts!{ f32 is 0.0 }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAllOpts {
    index: String,
}
