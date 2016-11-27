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

use commands::*;
use serde_json::value::ToJson;

macro_rules! implement {
    ($cmd:ty => $dt:ident) => {
        impl $cmd for types::$dt {}
        impl<O> $cmd for types::WithOpts<types::$dt, O> where O: Default + ToJson + Clone {}
    }
}

implement!{ Table => Db }
implement!{ Get => Table }
implement!{ GetAll => Table }
implement!{ Changes => Table }
implement!{ Changes => Stream }
implement!{ Changes => ObjectSelection }
