//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quick_error;
extern crate r2d2;
extern crate scram;
extern crate byteorder;
extern crate bufstream;
extern crate uuid;

pub mod conn;
pub mod proto;
pub mod types;
pub mod errors;

use std::collections::BTreeMap;

use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
use proto::{
    Term, Datum,
    Term_TermType as TT,
    Term_AssocPair as TA,
    Datum_DatumType as DT,
};

pub type Result<T> = ::std::result::Result<T, errors::Error>;

pub trait IsDatum {
    fn is_datum(&self) -> bool;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Encode {
    fn encode(&self) -> String;
}

impl IsDatum for Term {
    fn is_datum(&self) -> bool {
        self.get_field_type() == TT::DATUM
    }
}

impl IsEmpty for Term {
    fn is_empty(&self) -> bool {
        *self == Term::new()
    }
}

impl Encode for Vec<TA> {
    fn encode(&self) -> String {
        let mut opts = String::from("{");
        for term in self {
            opts.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
        }
        opts = opts.trim_right_matches(",").to_string();
        opts.push_str("}");
        opts
    }
}

impl Encode for Datum {
    fn encode(&self) -> String {
        match self.get_field_type() {
            DT::R_NULL => {
                String::from("null")
            },
            DT::R_BOOL => {
                format!("{}", self.get_r_bool())
            },
            DT::R_NUM => {
                format!("{}", self.get_r_num())
            },
            DT::R_STR => {
                format!("\"{}\"", self.get_r_str())
            },
            DT::R_ARRAY => {
                let mut args = format!("[{},[", TT::MAKE_ARRAY.value());
                for term in self.get_r_array() {
                    args.push_str(&format!("{},", term.encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("]]");
                args
            },
            DT::R_OBJECT => {
                let mut args = String::from("{");
                for term in self.get_r_object() {
                    args.push_str(&format!("\"{}\":{},", term.get_key(), term.get_val().encode()));
                }
                args = args.trim_right_matches(",").to_string();
                args.push_str("}");
                args
            },
            DT::R_JSON => {
                unimplemented!();
            },
        }
    }
}

impl Encode for Term {
    fn encode(&self) -> String {
        let mut res = Vec::new();
        if !self.is_datum() {
            res.push(format!("[{}", self.get_field_type().value()));
        }
        if self.has_datum() {
            let datum = self.get_datum();
            res.push(datum.encode());
        }
        let terms = self.get_args();
        if !terms.is_empty() {
            let mut args = String::from("[");
            for term in terms {
                args.push_str(&format!("{},", term.encode()));
            }
            args = args.trim_right_matches(",").to_string();
            args.push_str("]");
            res.push(args);
        }
        let opts = self.clone().take_optargs().into_vec();
        if !opts.is_empty() {
            res.push(format!("{}", opts.encode()));
        }
        let mut res = res.join(",");
        if !self.is_datum() {
            res.push_str("]");
        }
        res
    }
}

impl<'a> From<BTreeMap<&'a str, Term>> for Term {
    fn from(t: BTreeMap<&'a str, Term>) -> Term {
        let mut term = Term::new();
        let mut args = Vec::new();
        for (name, arg) in t.into_iter() {
            let mut obj = TA::new();
            obj.set_key(name.into());
            obj.set_val(arg);
            args.push(obj);
        }
        let obj = RepeatedField::from_vec(args);
        term.set_optargs(obj);
        term
    }
}

impl From<Vec<Term>> for Term {
    fn from(t: Vec<Term>) -> Term {
        let mut term = Term::new();
        term.set_field_type(TT::MAKE_ARRAY);
        let arr = RepeatedField::from_vec(t);
        term.set_args(arr);
        term
    }
}
