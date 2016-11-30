use types;

use serde_json::value::ToJson;

use proto::{
    Term_TermType as TermType,
};

use types::Command as Cmd;

pub use types::{
    WithTableOpts, WithChangesOpts, WithGetAllOpts
};

#[allow(non_upper_case_globals)]
pub const r: Command = Command;

impl Client for Command {}

pub struct Command;

#[allow(dead_code)]
impl Command {
    pub fn db<T>(&self, arg: T) -> types::Db where
        T: Into<types::String>
        {
            Cmd::new(TermType::DB, None)
                .with_args(arg.into().into())
                .into()
        }

    pub fn uuid(&self) -> types::String {
        Cmd::new(TermType::UUID, None)
            .into()
    }

    pub fn map<T, F>(&self, arg: T, func: F) -> types::Stream where
        T: Into<types::String>,
        F: FnOnce(types::String),
        {
            Cmd::new(TermType::MAP, None)
                .with_args(arg.into().into())
                .into()
        }

    pub fn map_arr<T, F>(&self, arg: T, func: F) -> types::Array where
        T: Into<types::String>,
        F: FnOnce(types::String),
        {
            Cmd::new(TermType::MAP, None)
                .with_args(arg.into().into())
                .into()
        }
}

pub trait Client {
    fn table<T>(&self, arg: T) -> types::WithOpts<types::Table, types::TableOpts> where
        T: Into<types::String>
        {
            r.db("test").table(arg)
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
    fn get_all<T>(&self, arg: T) -> types::WithOpts<types::StreamSelection, types::GetAllOpts>
        where T: Into<types::SecondaryKey>,
              types::GetAllOpts: Default + ToJson + Clone
    {
        Cmd::new(TermType::GET_ALL, Some(self.clone().into()))
            .with_args(arg.into().into())
            .into()
    }
}

pub trait GetField where Self: types::DataType {
    fn get_field<T, O>(&self, arg: T) -> O
        where T: Into<types::String>, O: types::DataType
        {
            Cmd::new(TermType::GET_FIELD, Some(self.clone().into()))
                .with_args(arg.into().into())
                .into()
        }
}

pub trait GetFieldArray where Self: types::DataType {
    fn get_field<T>(&self, arg: T) -> types::Array
        where T: Into<types::String>
        {
            Cmd::new(TermType::GET_FIELD, Some(self.clone().into()))
                .with_args(arg.into().into())
                .into()
        }
}
