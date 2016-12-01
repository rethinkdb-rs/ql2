use std::string::String as StdString;

use types;

use serde_json::value::{ToJson, Value};

use proto::{
    Term_TermType as TermType,
};

use types::Command as Cmd;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[allow(non_upper_case_globals)]
pub const r: Client = Client;

impl RootCommand for Client {}

pub trait RootCommand {
    fn table<T>(&self, arg: T) -> Command<types::Table, TableOpts> where
        T: Into<types::String>
        {
            r.db("test").table(arg)
        }
}

#[derive(Debug, Clone)]
pub struct Command<T, O>(T, Option<O>);

pub struct Client;

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

macro_rules! Root {
    () => {None as Option<&Command<types::Null, ()>>}
}

macro_rules! NoArg {
    () => {None as Option<types::Null>}
}

#[allow(dead_code)]
impl Client {
    pub fn db<T>(&self, arg: T) -> Command<types::Db, ()> where
        T: Into<types::String>
        {
            Cmd::make(TermType::DB, Some(arg.into()), None, Root!())
        }

    pub fn uuid(&self) -> Command<types::String, ()>
    {
        Cmd::make(TermType::UUID, NoArg!(), None, Root!())
    }
}

#[allow(dead_code)]
impl<O> Command<types::Db, O>
where O: ToJson + Clone
{
    pub fn table<T>(&self, arg: T) -> Command<types::Table, TableOpts> where
        T: Into<types::String>
        {
            Cmd::make(TermType::TABLE, Some(arg.into()), Some(TableOpts::default()), Some(self))
        }
}

#[allow(dead_code)]
impl<O> Command<types::Table, O>
where O: ToJson + Clone
{
    pub fn get<T>(&self, arg: T) -> Command<types::ObjectSelection, ()> where
        T: Into<types::PrimaryKey>,
    {
        Cmd::make(TermType::GET, Some(arg.into()), None, Some(self))
    }

    pub fn changes(&self) -> Command<types::Stream, ChangesOpts<bool>> where
        ChangesOpts<bool>: Default + ToJson + Clone
        {
            let opts: ChangesOpts<bool> = Default::default();
            Cmd::make(TermType::CHANGES, NoArg!(), Some(opts), Some(self))
        }

    pub fn get_all<T>(&self, arg: T) -> Command<types::StreamSelection, GetAllOpts> where
        T: Into<types::SecondaryKey>,
        GetAllOpts: Default + ToJson + Clone
        {
            Cmd::make(TermType::GET_ALL, Some(arg.into()), Some(GetAllOpts::default()), Some(self))
        }
}

#[allow(dead_code)]
impl<O> Command<types::Stream, O>
where O: ToJson + Clone
{
    pub fn changes(&self) -> Command<types::Stream, ChangesOpts<bool>> where
        ChangesOpts<bool>: Default + ToJson + Clone
        {
            let opts: ChangesOpts<bool> = Default::default();
            Cmd::make(TermType::CHANGES, NoArg!(), Some(opts), Some(self))
        }

    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<types::Stream>,
        {
            Cmd::make(TermType::MAP, Some(arg.into()), None, Some(self))
        }
}

#[allow(dead_code)]
impl<O> Command<types::StreamSelection, O>
where O: ToJson + Clone
{
    pub fn changes(&self) -> Command<types::Stream, ChangesOpts<bool>> where
        ChangesOpts<bool>: Default + ToJson + Clone
        {
            let opts: ChangesOpts<bool> = Default::default();
            Cmd::make(TermType::CHANGES, NoArg!(), Some(opts), Some(self))
        }
}

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
impl<O> Command<types::ObjectSelection, O>
where O: ToJson + Clone
{
    pub fn changes(&self) -> Command<types::Stream, ChangesOpts<bool>> where
        ChangesOpts<bool>: Default + ToJson + Clone
        {
            let opts: ChangesOpts<bool> = Default::default();
            Cmd::make(TermType::CHANGES, NoArg!(), Some(opts), Some(self))
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

    pub fn map<T>(&self, arg: T) -> Command<types::Array, ()> where
        T: Into<types::Array>,
        {
            Cmd::make(TermType::MAP, Some(arg.into()), None, Some(self))
        }
}

#[allow(dead_code)]
impl<T> Command<T, TableOpts>
{
    pub fn read_mode(mut self, arg: ReadMode) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.read_mode = arg;
        }
        self
    }

    pub fn identifier_format(mut self, arg: IdentifierFormat) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.identifier_format = arg;
        }
        self
    }
}

pub trait SquashArg where Self: ToJson + Clone {}
impl SquashArg for bool {}
impl SquashArg for f32 {}

#[allow(dead_code)]
impl<T, A> Command<T, ChangesOpts<A>>
where A: SquashArg, ChangesOpts<A>: Default + ToJson + Clone
{
    pub fn squash<B>(self, arg: B) -> Command<T, ChangesOpts<B>>
        where B: SquashArg, ChangesOpts<B>: Default + ToJson + Clone
        {
            let o = self.1.unwrap_or(Default::default());
            let opts = ChangesOpts {
                squash: arg,
                changefeed_queue_size: o.changefeed_queue_size,
                include_initial: o.include_initial,
                include_states: o.include_states,
                include_offsets: o.include_offsets,
                include_types: o.include_types,
            };
            Command(self.0, Some(opts))
        }

    pub fn changefeed_queue_size(mut self, arg: u64) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.changefeed_queue_size = arg;
        }
        self
    }

    pub fn include_initial(mut self, arg: bool) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.include_initial = arg;
        }
        self
    }

    pub fn include_states(mut self, arg: bool) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.include_states = arg;
        }
        self
    }

    pub fn include_offsets(mut self, arg: bool) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.include_offsets = arg;
        }
        self
    }

    pub fn include_types(mut self, arg: bool) -> Self {
        if let Some(ref mut opts) = self.1 {
            opts.include_types = arg;
        }
        self
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
