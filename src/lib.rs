//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate quick_error;
extern crate r2d2;
extern crate scram;
extern crate byteorder;
extern crate bufstream;

pub mod conn;
pub mod proto;
pub mod types;
pub mod errors;

pub type Result<T> = ::std::result::Result<T, errors::Error>;
