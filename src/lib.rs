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

use serde_json::value::{Value, Map};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
use proto::{
    Term, Datum,
    Term_TermType as TT,
    Term_AssocPair as TA,
    Datum_DatumType as DT,
};

pub type Result<T> = ::std::result::Result<T, errors::Error>;

type Array = Vec<Term>;

type Object = Map<String, Term>;

pub trait IsDatum {
    fn is_datum(&self) -> bool;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Encode {
    fn encode(&self) -> String;
}

pub trait Decode {
    fn decode(&self) -> Value;
}

#[derive(Debug, Clone)]
pub struct QueryInfo {
    db_set: bool,
    query: String,
    op: String,
}

impl QueryInfo {
    pub fn db_set(&self) -> bool {
        self.db_set
    }

    /*
    fn query(&self) -> String {
        self.query.to_string()
    }

    fn cmd_type(&self) -> String {
        self.op.to_string()
    }
    */
}

pub trait Info {
    fn info(&self) -> QueryInfo;
}

impl Info for Term {
    fn info(&self) -> QueryInfo {
        let mut inf = QueryInfo {
            db_set: false,
            query: String::new(),
            op: String::from("read"),
        };
        let cmd = self.get_field_type()
            .descriptor()
            .name();
        if cmd == "INSERT" {
            inf.op = String::from("INSERT");
        } else if cmd == "CHANGES" {
            inf.op = String::from("CHANGES");
        } else if cmd == "DB" {
            inf.db_set = true;
        }
        inf.query.push_str(&format!(".{}(arg)", cmd));
        let terms = self.get_args();
        if !terms.is_empty() {
            for term in terms {
                let tinf = term.info();
                if tinf.db_set {
                    inf.db_set = true;
                }
                inf.query.push_str(&tinf.query);
                if tinf.op != "read" {
                    inf.op = tinf.op;
                }
            }
        }
        inf
    }
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
        let arr = RepeatedField::from_vec(t);
        term.set_args(arr);
        term
    }
}
