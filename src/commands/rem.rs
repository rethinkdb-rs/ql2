use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::Command;
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Number, O>
where O: ToJson + Clone
{
    pub fn rem<T>(&self, arg: T) -> Command<types::Number, ()> where
        T: Into<types::Number>
        {
            Cmd::make(
                TermType::MOD,
                Some(arg.into()),
                None,
                Some(self),
                )
        }
}
