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
            Cmd::new(TermType::DB, None)
                .with_args(arg.into().into())
                .into()
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
            Cmd::new(TermType::TABLE, Some(self.clone().into()))
                .with_args(arg.into().into())
                .into()
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

pub trait Get where Self: types::DataType {
    fn get<T>(&self, arg: T) -> types::ObjectSelection
        where T: Into<types::PrimaryKey>,
    {
        Cmd::new(TermType::GET, Some(self.clone().into()))
            .with_args(arg.into().into())
            .into()
    }
}

pub trait GetAll where Self: types::DataType {
    fn get_all<T>(&self, arg: T) -> types::StreamSelection
        where T: Into<types::PrimaryKey>,
    {
        Cmd::new(TermType::GET_ALL, Some(self.clone().into()))
            .with_args(arg.into().into())
            .into()
    }
}
