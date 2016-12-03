#![allow(dead_code)]

use types;
use types::Command as Cmd;
use proto::{
    Term,
    Term_TermType as TermType,
};
use super::Command;
use serde_json::value::ToJson;

impl<O> Command<types::Table, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<StreamMapArg>,
    {
        let arg: types::Stream = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Stream, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Stream, ()> where
        T: Into<StreamMapArg>,
    {
        let arg: types::Stream = arg.into().into();
        Cmd::make(TermType::MAP, Some(arg), None, Some(self))
    }
}

impl<O> Command<types::Array, O> where
O: ToJson + Clone
{
    pub fn map<T>(&self, arg: T) -> Command<types::Array, ()> where
        T: Into<types::Array>,
    {
        Cmd::make(TermType::MAP, Some(arg.into()), None, Some(self))
    }
}

pub struct StreamMapArg(Term);

impl From<StreamMapArg> for types::Stream {
    fn from(t: StreamMapArg) -> types::Stream {
        From::from(t.0)
    }
}

/*
impl<F, T, O> From<F> for StreamMapArg where
T: types::DataType,
O: ToJson + Clone,
F: FnOnce(Command<types::Object, ()>) -> Command<T, O>
{
    fn from(t: F) -> StreamMapArg {
        let res = t(var!());
        let term = func!(res.into());
        StreamMapArg(term)
    }
}
*/

impl<F, T, O> From<F> for StreamMapArg where
T: types::DataType,
O: ToJson + Clone,
F: Fn(Command<types::Object, ()>) -> Command<T, O>
{
    fn from(t: F) -> StreamMapArg {
        let res = t(var!());
        let term = func!(res.into());
        StreamMapArg(term)
    }
}
