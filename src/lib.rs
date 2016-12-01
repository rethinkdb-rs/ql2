//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde;
extern crate uuid;
extern crate serde_json;
#[macro_use]
extern crate quick_error;
extern crate scram;
extern crate r2d2;
extern crate futures;

pub mod proto;
pub mod types;
pub mod errors;
pub mod commands;
