use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, TableOpts};
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
