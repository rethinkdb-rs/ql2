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

use types::Command as Cmd;
use serde_json::value::ToJson;

type CmdWithOpts<T, O> = Cmd<types::WithOpts<T, O>>;

macro_rules! implement {
    ($cmd:ident for $dt:ident) => {
        impl commands::$cmd for Cmd<types::$dt> {}
        impl<O> commands::$cmd for CmdWithOpts<types::$dt, O> where O: Default + ToJson {}
    }
}

implement!{ Table for Db }
implement!{ Changes for Table }
implement!{ Changes for Stream }
implement!{ Changes for ObjectSelection }
