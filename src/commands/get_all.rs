use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, GetAllOpts};
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Table, O>
where O: ToJson + Clone
{
    pub fn get_all<T>(&self, arg: T) -> Command<types::StreamSelection, GetAllOpts> where
        T: Into<types::SecondaryKey>,
        GetAllOpts: Default + ToJson + Clone
        {
            Cmd::make(TermType::GET_ALL, Some(vec![arg.into()]), Some(GetAllOpts::default()), Some(self))
        }
}

#[allow(dead_code)]
impl<T> Command<T, GetAllOpts>
{
    pub fn index(mut self, arg: &str) -> Self {
        match self.1 {
            Some(ref mut opts) => {
                opts.index = arg.to_string();
            },
            None => {
                let opts = GetAllOpts {
                    index: arg.to_string(),
                };
                self.1 = Some(opts);
            }
        }
        self
    }
}
