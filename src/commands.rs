use types;

use proto::{
    Term_TermType as TermType,
};

macro_rules! none {
    () => {None as Option<types::Null>}
}

pub trait Command where Self: Sized {
    fn db<T>(self, arg: T) -> types::Db
        where T: Into<types::String>
        {
            let mut output = types::Command::new(TermType::DB, none!());
            output.set_args(arg.into());
            output.into()
        }

    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>;

    fn uuid(self) -> types::String {
        types::Command::new(TermType::UUID, none!()).into()
    }
}

pub trait Db where Self: types::DataType {
    fn table<T>(self, arg: T) -> types::WithOpts<types::Table, types::TableOpts>
        where T: Into<types::String>
    {
        let mut output = types::Command::new(TermType::TABLE, Some(self));
        output.set_args(arg.into());
        types::WithOpts::new(output.into(), None)
    }
}

pub trait Stream where Self: types::DataType {
    fn changes(self) -> types::WithOpts<types::Stream, types::ChangesOpts> {
        let output: types::Stream = types::Command::new(TermType::CHANGES, Some(self))
            .into();
        types::WithOpts::new(output, None)
    }
}

pub trait ObjectSelection where Self: types::DataType {
    fn changes(self) -> types::WithOpts<types::Stream, types::ChangesOpts> {
        let output: types::Stream = types::Command::new(TermType::CHANGES, Some(self))
            .into();
        types::WithOpts::new(output, None)
    }
}
