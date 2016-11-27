use types;

use serde_json::value::ToJson;

use proto::{
    Term_TermType as TermType,
};

use types::Command as Cmd;

pub use types::{
    WithTableOpts, WithChangesOpts
};

pub trait Command where Self: Sized {
    fn db<T>(&self, arg: T) -> types::Db
        where T: Into<types::String>
        {
            let mut cmd = Cmd::new(TermType::DB, None);
            cmd.with_args(arg.into().into());
            cmd.into()
        }

    fn table<T>(&self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            self.db("test").table(arg)
        }

    fn uuid(&self) -> types::String {
        Cmd::new(TermType::UUID, None)
            .into()
    }
}

pub trait Table where Self: types::DataType {
    fn table<T>(&self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
        {
            let mut cmd = Cmd::new(TermType::TABLE, Some(self.clone().into()));
            cmd.with_args(arg.into().into());
            cmd.into()
        }
}

pub trait Changes where Self: types::DataType {
    fn changes(&self) -> types::WithOpts<types::Stream, types::ChangesOpts<bool>>
        where types::ChangesOpts<bool>: Default + ToJson + Clone
        {
            Cmd::new(TermType::CHANGES, Some(self.clone().into()))
                .into()
        }
}
