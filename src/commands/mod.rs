#![macro_use]

macro_rules! Root {
    () => {None as Option<&Command<types::Null, ()>>}
}

macro_rules! NoArg {
    () => {None as Option<types::Null>}
}

pub mod db;
pub mod table;
pub mod uuid;
pub mod get;
pub mod get_all;
pub mod changes;
pub mod map;
pub mod get_field;
pub mod rem;

use std::string::String as StdString;

use types;

use serde_json::value::{ToJson, Value};

use proto::{
    Term_TermType as TermType,
};

use types::Command as Cmd;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[allow(non_upper_case_globals)]
pub const r: Client = Command((), None);

impl RootCommand for Client {}

pub trait RootCommand {
    fn table<T>(&self, arg: T) -> Command<types::Table, TableOpts> where
        T: Into<types::String>
        {
            r.db("test").table(arg)
        }
}

#[derive(Debug)]
pub struct Command<T, O>(T, Option<O>);

pub type Client = Command<(), ()>;

impl Cmd {
    pub fn make<A, T, O, PT, PO>(typ: TermType, arg: Option<A>, opts: Option<O>, cmd: Option<&Command<PT, PO>>) -> Command<T, O>
        where
        A: types::DataType,
        T: types::DataType,
        O: ToJson + Clone,
        PT: types::DataType,
        PO: ToJson + Clone
        {
            let (prev_cmd, prev_opts) = match cmd {
                Some(cmd) => (Some(cmd.0.clone().into()), cmd.1.clone()),
                None => (None, None),
            };
            let mut dt = Cmd::new(typ, prev_cmd);
            if let Some(arg) = arg {
                dt = dt.with_args(arg.into());
            }
            if let Some(opt) = prev_opts {
                let obj = types::Object::from(opt);
                dt = dt.with_opts(obj);
            }
            Command(dt.into(), opts)
        }
}
