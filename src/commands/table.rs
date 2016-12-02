use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, TableOpts, ReadMode, IdentifierFormat};
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Db, O>
where O: ToJson + Clone
{
    pub fn table<T>(&self, arg: T) -> Command<types::Table, TableOpts> where
        T: Into<types::String>
        {
            Cmd::make(
                TermType::TABLE,
                Some(arg.into()),
                Some(TableOpts::default()),
                Some(self),
                )
        }
}

#[allow(dead_code)]
impl<T> Command<T, TableOpts>
{
    pub fn read_mode(mut self, arg: ReadMode) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.read_mode = arg;
        }
        self
    }

    pub fn identifier_format(mut self, arg: IdentifierFormat) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.identifier_format = arg;
        }
        self
    }
}
