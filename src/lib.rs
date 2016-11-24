//! RethinkDB protocol implementation in Rust

extern crate protobuf;

pub mod proto;
pub mod types;
pub mod commands;

impl commands::Db for types::Db {}

impl commands::Stream for types::Stream {}

impl commands::ObjectSelection for types::Selection<types::Object> {}
