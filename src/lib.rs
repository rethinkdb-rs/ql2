//! RethinkDB protocol implementation in Rust

#![no_std]

pub use prost;

include!(concat!(env!("OUT_DIR"), "/ql2.rs"));
