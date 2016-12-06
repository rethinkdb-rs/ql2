#![allow(dead_code)]

use types;
use types::Command as Cmd;
use proto::{Term, Term_TermType as TermType};
use super::{Command, Arg};
use serde_json::value::ToJson;

impl<O> Command<types::Table, O>
    where O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()>
        where T: Into<MapArg<types::Stream>>
    {
        let arg: Vec<types::Stream> = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Stream, O>
    where O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()>
        where T: Into<MapArg<types::Stream>>
    {
        let arg: Vec<types::Stream> = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Array, O>
    where O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Array, ()>
        where T: Into<types::Array>
    {
        Cmd::make(TermType::MAP, Some(vec![arg.into()]), None, Some(self))
    }
}

pub struct MapArg<T>(Vec<T>);

impl From<MapArg<types::Stream>> for Vec<types::Stream> {
    fn from(t: MapArg<types::Stream>) -> Vec<types::Stream> {
        t.0
    }
}

impl<F, T, O> From<F> for MapArg<types::Stream>
    where T: types::DataType,
          O: ToJson + Clone,
          F: Fn(Arg) -> Command<T, O>
{
    fn from(t: F) -> MapArg<types::Stream> {
        let res = t(var!());
        let term = func!(res.into());
        MapArg(vec![term.into()])
    }
}

pub trait Stream: types::DataType {}

impl Stream for types::Table {}

impl<F, T, O> From<F> for types::Function
    where T: types::DataType,
          O: ToJson + Clone,
          F: Fn(Arg, Arg) -> Command<T, O>,
{
    fn from(t: F) -> types::Function {
        let res = t(var!(), var!());
        let term = func!(res.into());
        From::from(term)
    }
}

impl<F, CT, CO> From<(Vec<Command<CT, CO>>, F)> for MapArg<types::Stream>
    where CT: Stream,
          CO: ToJson + Clone,
          F: Into<types::Function>
{
    fn from(t: (Vec<Command<CT, CO>>, F)) -> MapArg<types::Stream> {
        let mut args = Vec::with_capacity(t.0.len()+1);
        for arg in t.0 {
            let arg: Term = arg.into();
            args.push(arg.into());
        }
        let func: Term = t.1.into().into();
        args.push(func.into());
        MapArg(args)
    }
}
