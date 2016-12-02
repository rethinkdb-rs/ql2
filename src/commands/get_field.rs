use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Object, O>
where O: ToJson + Clone
{
    pub fn get_field<T, V>(&self, arg: T) -> Command<V, ()>
        where T: Into<types::String>, V: types::DataType
        {
            Cmd::make(TermType::GET_FIELD, Some(arg.into()), None, Some(self))
        }
}

#[allow(dead_code)]
impl<O> Command<types::Array, O>
where O: ToJson + Clone
{
    pub fn get_field<T>(&self, arg: T) -> Command<types::Array, ()>
        where T: Into<types::String>
        {
            Cmd::make(TermType::GET_FIELD, Some(arg.into()), None, Some(self))
        }
}
