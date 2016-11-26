use types;

use serde_json::value::ToJson;

use proto::{
    Term,
    Term_TermType as TermType,
};

use types::Command as Cmd;

type CmdWithOpts<T, O> = Cmd<types::WithOpts<T, O>>;

macro_rules! none {
    () => {None as Option<types::Null>}
}

pub trait Command where Self: Sized {
    fn db<T>(self, arg: T) -> Cmd<types::Db>
        where T: Into<types::String>
        {
            Cmd::new::<Term>(TermType::DB, none!())
                .with_args(arg.into())
        }

    fn table<T>(self, arg: T) -> CmdWithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            self.db("test").table(arg)
        }

    fn uuid(self) -> Cmd<types::String> {
        Cmd::new(TermType::UUID, none!())
    }
}

pub trait Table where Self: Sized + From<Term> + Into<Term> {
    fn table<T>(self, arg: T) -> CmdWithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            Cmd::new::<Term>(TermType::TABLE, Some(self))
                .with_args(arg.into())
        }
}

pub trait Changes where Self: Sized + From<Term> + Into<Term> {
    fn changes<T>(self) -> CmdWithOpts<types::Stream, types::ChangesOpts<T>>
        where types::ChangesOpts<T>: Default + ToJson
        {
            Cmd::new(TermType::CHANGES, Some(self))
        }
}
