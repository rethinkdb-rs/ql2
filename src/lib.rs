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

impl Term {
    fn is_empty(&self) -> bool {
        *self == Term::new()
    }
}

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
    ($T:expr, $cmd:expr, $args:expr, $opts:expr) => {{
        let mut term = Term::new();
        term.set_field_type($T);
        let mut args = Vec::new();
        if !$cmd.is_empty() {
            args.push($cmd);
        }
        if let Some(list) = $args {
            args.extend(list);
        }
        /*
        if let Some(_opt) = $opts {
            unimplemented!();
        }
        */
        if !args.is_empty() {
            let args = RepeatedField::from_vec(args);
            term.set_args(args);
        }
        From::from(term)
    }}
}

macro_rules! closure_par {
    () => {{
        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num(1.0);
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(id);
        // VAR
        let mut var = Term::new();
        var.set_field_type(TT::VAR);
        let args = RepeatedField::from_vec(vec![datum]);
        var.set_args(args);
        From::from(var)
    }}
}

macro_rules! closure_arg {
    ($res:expr) => {{
        // ID
        let mut id = Datum::new();
        id.set_field_type(DT::R_NUM);
        id.set_r_num(1.0);
        // ARRAY
        let mut array = Datum::new();
        array.set_field_type(DT::R_ARRAY);
        let args = RepeatedField::from_vec(vec![id]);
        array.set_r_array(args);
        // DATUM
        let mut datum = Term::new();
        datum.set_field_type(TT::DATUM);
        datum.set_datum(array);
        // FUNC
        let mut func = Term::new();
        func.set_field_type(TT::FUNC);
        let args = RepeatedField::from_vec(vec![datum, $res]);
        func.set_args(args);
        func
    }}
}

pub trait Command
    where Self: Sized + From<Term> + Into<Term>
{
    fn db<T: ToTerm>(self, arg: T) -> Self {
        let cmd = self.into();
        let arg = arg.to_term();
        command!(TT::DB, cmd, Some(vec![arg]), None)
    }

    fn table<T: ToTerm>(self, arg: T) -> Self {
        let cmd = self.into();
        let arg = arg.to_term();
        command!(TT::TABLE, cmd, Some(vec![arg]), None)
    }

    fn get_field<T: ToTerm>(self, arg: T) -> Self {
        let cmd = self.into();
        let arg = arg.to_term();
        command!(TT::GET_FIELD, cmd, Some(vec![arg]), None)
    }

    fn map<F>(self, func: F) -> Self
        where F: Fn(Self) -> Self
    {
        let cmd = self.into();
        let arg = func(closure_par!()).into();
        command!(TT::MAP, cmd, Some(vec![arg]), None)
    }

    fn array(self, arg: Vec<&ToTerm>) -> Self {
        let cmd = self.into();
        let args: Vec<Term> = arg.iter()
            .map(|a| a.to_term())
            .collect();
        command!(TT::MAKE_ARRAY, cmd, Some(args), None)
    }
}

#[test]
fn test_commands_can_be_chained() {
    impl Command for Term { }
    let r = Term::new();
    let term = r.db("heroes").table("marvel").map(|row| row.get_field("first_appearance"));
    panic!(format!("{:?}", term));
}
