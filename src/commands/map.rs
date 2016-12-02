use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Stream, O>
where O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<types::Stream>,
        {
            Cmd::make(TermType::MAP, Some(arg.into()), None, Some(self))
        }
}

#[allow(dead_code)]
impl<O> Command<types::Array, O>
where O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Array, ()> where
        T: Into<types::Array>,
        {
            Cmd::make(TermType::MAP, Some(arg.into()), None, Some(self))
        }
}
