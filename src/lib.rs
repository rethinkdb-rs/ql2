//! RethinkDB protocol implementation in Rust

extern crate protobuf;

mod proto;
pub mod types;

pub use proto::{
    VersionDummy_Version as Version,
    Term_TermType as TermType,
    Query_QueryType as QueryType,
    Response_ResponseType as ResponseType,
    Response_ErrorType as ErrorType,
};

macro_rules! none {
    () => {None as Option<types::Null>}
}

// ROOT COMMAND
pub trait RootCommand where Self: Sized {
    fn db<T>(self, arg: T) -> types::Db
        where T: Into<types::String>
        {
            let mut output = types::Command::new(TermType::DB, none!());
            output.set_args(arg.into());
            output.into()
        }

    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>;

    fn uuid(self) -> types::String {
        types::Command::new(TermType::UUID, none!()).into()
    }
}

// DB
pub trait Db where Self: types::DataType {
    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
    {
        let mut output = types::Command::new(TermType::TABLE, Some(self));
        output.set_args(arg.into());
        types::WithOpts::new(output.into(), None)
    }
}

impl Db for types::Db {}

// STREAM
pub trait Stream where Self: types::DataType {
    fn changes(self) -> types::WithOpts<types::Stream, types::ChangesOpts> {
        let output: types::Stream = types::Command::new(TermType::CHANGES, Some(self))
            .into();
        types::WithOpts::new(output, None)
    }
}

impl Stream for types::Stream {}

// OBJECT SELECTION
pub trait ObjectSelection where Self: types::DataType {
    fn changes(self) -> types::WithOpts<types::Stream, types::ChangesOpts> {
        let output: types::Stream = types::Command::new(TermType::CHANGES, Some(self))
            .into();
        types::WithOpts::new(output, None)
    }
}

impl ObjectSelection for types::Selection<types::Object> {}
