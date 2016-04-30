//! RethinkDB protocol implementation in Rust
//!
//! This native implementation of the RethinkDB protocol is auto-generated from [ql2.proto] using
//! [protobuf]. The implementation itself is in the [proto] module. Using it is as straight forward
//! as:
//!
//! [ql2.proto]: https://github.com/rethinkdb/rethinkdb/blob/v2.3.1/src/rdb_protocol/ql2.proto
//! [protobuf]: https://crates.io/crates/protobuf
//! [proto]: proto/index.html
//! ```
//! extern crate ql2;
//!
//! use ql2::proto;
//!
//! # fn main() {
//! let version = proto::VersionDummy_Version::V1_0;
//! # }
//! ```

#![doc(html_root_url="http://rust-rethinkdb.github.io/ql2/1.0.x")]

extern crate protobuf;

pub mod proto;
