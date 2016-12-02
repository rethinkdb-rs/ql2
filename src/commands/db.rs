use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, Client};

#[allow(dead_code)]
impl Client {
    pub fn db<T>(&self, arg: T) -> Command<types::Db, ()> where
        T: Into<types::String>
        {
            Cmd::make(
                TermType::DB,
                Some(arg.into()),
                None,
                Root!(),
                )
        }
}
