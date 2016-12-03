#![allow(dead_code)]

use types;
use types::Command as Cmd;
use proto::{
    Term,
    Term_TermType as TermType,
};
use super::{Command, Arg};
use serde_json::value::ToJson;

impl<O> Command<types::Table, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<MapArg<types::Stream>>,
    {
        let arg: Vec<types::Stream> = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Stream, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<MapArg<types::Stream>>,
    {
        let arg: Vec<types::Stream> = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Array, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Array, ()> where
        T: Into<types::Array>,
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

impl<F, T, O> From<F> for MapArg<types::Stream> where
T: types::DataType,
O: ToJson + Clone,
F: Fn(Arg) -> Command<T, O>
{
    fn from(t: F) -> MapArg<types::Stream> {
        let res = t(var!());
        let term = func!(res.into());
        MapArg(vec![term.into()])
    }
}

pub trait Stream : types::DataType {}

impl Stream for types::Table {}

impl<F, CT, CO, T, O> From<(Command<CT, CO>, F)> for MapArg<types::Stream> where
CT: Stream,
CO: ToJson + Clone,
T: types::DataType,
O: ToJson + Clone,
F: Fn(Arg, Arg) -> Command<T, O>
{
    fn from(t: (Command<CT, CO>, F)) -> MapArg<types::Stream> {
        let arg: Term = t.0.into();
        let res = t.1(var!(), var!());
        let term = func!(res.into());
        MapArg(vec![arg.into(), term.into()])
    }
}
