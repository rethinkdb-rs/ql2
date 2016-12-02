use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::{Command, ChangesOpts};
use serde_json::value::ToJson;

#[allow(dead_code)]
impl<O> Command<types::Table, O>
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
impl<O> Command<types::Stream, O>
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
