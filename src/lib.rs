//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde_json;

pub mod proto;

use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;
use proto::{
    Term, Datum,
    Datum_DatumType as DT,
    Term_TermType as TT,
};

pub trait ToTerm {
    fn to_term(&self) -> Term;
}

impl<T: ToJson> ToTerm for T {
    fn to_term(&self) -> Term {
        // Datum
        let mut datum = Datum::new();
        match self.to_json() {
            Value::String(val) => {
                datum.set_field_type(DT::R_STR);
                datum.set_r_str(val);
            },
            Value::Bool(val) => {
                datum.set_field_type(DT::R_BOOL);
                datum.set_r_bool(val);
            },
            Value::I64(val) => {
                datum.set_field_type(DT::R_NUM);
                datum.set_r_num(val as f64);
            },
            Value::U64(val) => {
                datum.set_field_type(DT::R_NUM);
                datum.set_r_num(val as f64);
            },
            Value::F64(val) => {
                datum.set_field_type(DT::R_NUM);
                datum.set_r_num(val);
            },
            Value::Array(_val) => {
                unimplemented!();
            },
            Value::Object(_val) => {
                unimplemented!();
            },
            Value::Null => {
                datum.set_field_type(DT::R_NULL);
            },
        }
        // Term
        let mut term = Term::new();
        term.set_field_type(TT::DATUM);
        term.set_datum(datum);
        term
    }
}

impl ToTerm for Term {
    fn to_term(&self) -> Term {
        self.clone()
    }
}

macro_rules! command {
    ($T:expr, $args:expr) => {{
        let mut term = Term::new();
        term.set_field_type($T);
        if !$args.is_empty() {
            let args = RepeatedField::from_vec($args);
            term.set_args(args);
        }
        From::from(term)
    }}
}

pub trait Command
    where Self: Sized + From<Term> + Into<Term>
{
    fn db<T: ToTerm>(self, arg: T) -> Self {
        let args = vec![
            arg.to_term(),
            ];
        command!(TT::DB, args)
    }

    fn table<T: ToTerm>(self, arg: T) -> Self {
        let args = vec![
            self.into(),
            arg.to_term(),
            ];
        command!(TT::TABLE, args)
    }

    fn object(self, arg: Vec<&ToTerm>) -> Self {
        let args: Vec<Term> = arg.iter()
            .map(|a| a.to_term())
            .collect();
        command!(TT::OBJECT, args)
    }
}

#[test]
fn test_commands_can_be_chained() {
    impl Command for Term { }
    let r = Term::new();
    r.db("heroes").table("marvel");
}
