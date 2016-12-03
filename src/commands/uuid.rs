use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, Client};

#[allow(dead_code)]
impl Client {
    pub fn uuid(&self) -> Command<types::String, ()> {
        Cmd::make(TermType::UUID, NoArg!(), None, Root!())
    }
}
