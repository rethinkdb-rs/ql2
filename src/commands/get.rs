use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Table, O> where
O: ToJson + Clone
{
    pub fn get<T>(&self, arg: T) -> Command<types::ObjectSelection, ()> where
        T: Into<types::PrimaryKey>,
    {
        Cmd::make(
            TermType::GET,
            Some(vec![arg.into()]),
            None,
            Some(self),
            )
    }
}
