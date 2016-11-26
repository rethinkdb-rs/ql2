use types;

use serde_json::value::ToJson;

use proto::{
    Term,
    Term_TermType as TermType,
};

use types::Command as Cmd;

macro_rules! none {
    () => {None as Option<types::Null>}
}

pub trait Command where Self: Sized {
    fn db<T>(self, arg: T) -> types::Db
        where T: Into<types::String>
        {
            Cmd::new(TermType::DB, none!())
                .with_args(arg.into())
                .into()
        }

    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            self.db("test").table(arg)
        }

    fn uuid(self) -> types::String {
        Cmd::new(TermType::UUID, none!())
                .into()
    }
}

pub trait Table where Self: Sized + From<Term> + Into<Term> {
    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            Cmd::new(TermType::TABLE, Some(self))
                .with_args(arg.into())
                .into()
        }
}

pub trait Changes where Self: Sized + From<Term> + Into<Term> {
    fn changes<T>(self) -> types::WithOpts<types::Stream, types::ChangesOpts<T>>
        where types::ChangesOpts<T>: Default + ToJson
        {
            Cmd::new(TermType::CHANGES, Some(self))
                .into()
        }
}
