//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde_json;

pub mod proto;

use std::collections::BTreeMap;

use serde_json::value::{Value, Map, ToJson};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
use proto::{
    Term, Datum,
    Term_TermType as TT,
    Term_AssocPair as TA,
    Datum_DatumType as DT,
    Datum_AssocPair as DA
};

macro_rules! command {
    ($T:expr, $cmd:expr, $args:expr) => {{
        let mut term = Term::new();
        term.set_field_type($T);
        let mut args = Vec::new();
        let cmd = $cmd.to();
        if !cmd.is_empty() {
            args.push(cmd);
        }
        if let Some(list) = $args {
            args.extend(list);
        }
        if !args.is_empty() {
            let args = RepeatedField::from_vec(args);
            term.set_args(args);
        }
        FromTerm::from(term)
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
        var
    }}
}

macro_rules! closure_arg {
    ($func:expr) => {{
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
        let res = $func(closure_par!());
        let args = RepeatedField::from_vec(vec![datum, res]);
        func.set_args(args);
        vec![func]
    }}
}

pub type Array = Vec<Value>;

pub type Object = Map<String, Value>;

pub trait IsDatum {
    fn is_datum(&self) -> bool;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait Encode {
    fn encode(&self) -> String;
}

pub trait ToTerm {
    fn to(&self) -> Term;
}

pub trait FromTerm {
    fn from(t: Term) -> Self;
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

impl Encode for Datum {
    fn encode(&self) -> String {
        match self.get_field_type() {
            DT::R_NULL => {
                unimplemented!();
            },
            DT::R_BOOL => {
                if self.has_r_bool() {
                    format!("{:?}", self.get_r_bool())
                } else {
                    unimplemented!();
                }
            },
            DT::R_NUM => {
                if self.has_r_num() {
                    format!("{}", self.get_r_num())
                } else {
                    unimplemented!();
                }
            },
            DT::R_STR => {
                if self.has_r_str() {
                    format!("{:?}", self.get_r_str())
                } else {
                    unimplemented!();
                }
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
                    args.push_str(&format!("{:?}: {},", term.get_key(), term.get_val().encode()));
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
        let mut res = String::new();
        if !self.is_datum() {
            res.push_str(&format!("[{},", self.get_field_type().value()));
        }
        let terms = self.get_args();
        if !terms.is_empty() {
            let mut args = String::from("[");
            for term in terms {
                args.push_str(&format!("{},", term.encode()));
            }
            args = args.trim_right_matches(",").to_string();
            args.push_str("]");
            res.push_str(&args);
        }
        if self.has_datum() {
            let datum = self.get_datum();
            res.push_str(&datum.encode());
        }
        if !self.is_datum() {
            res.push_str("]");
        }
        res
    }
}

impl<T: ToJson> ToTerm for T {
    fn to(&self) -> Term {
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
            Value::Array(val) => {
                datum.set_field_type(DT::R_ARRAY);
                let args: Vec<Datum> = val.iter()
                    .map(|a| a.to().take_datum())
                    .collect();
                let arr = RepeatedField::from_vec(args);
                datum.set_r_array(arr);
            },
            Value::Object(val) => {
                datum.set_field_type(DT::R_OBJECT);
                let args: Vec<DA> = val.into_iter()
                    .map(|(name, arg)| {
                        let mut obj = DA::new();
                        obj.set_key(name.into());
                        obj.set_val(arg.to().take_datum());
                        obj
                    })
                    .collect();
                let obj = RepeatedField::from_vec(args);
                datum.set_r_object(obj);
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
    fn to(&self) -> Term {
        self.clone()
    }
}

impl FromTerm for Term {
    fn from(t: Term) -> Term {
        t
    }
}

pub trait Command : FromTerm + ToTerm {
    fn expr<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        FromTerm::from(arg.to())
    }

    fn db<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB, self, Some(vec![arg.to()]))
    }

    fn db_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB_CREATE, self, Some(vec![arg.to()]))
    }

    fn db_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB_DROP, self, Some(vec![arg.to()]))
    }

    fn table<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE, self, Some(vec![arg.to()]))
    }

    fn table_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE_CREATE, self, Some(vec![arg.to()]))
    }

    fn table_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE_DROP, self, Some(vec![arg.to()]))
    }

    fn index_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INDEX_CREATE, self, Some(vec![arg.to()]))
    }

    fn index_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INDEX_DROP, self, Some(vec![arg.to()]))
    }

    fn replace<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::REPLACE, self, Some(vec![arg.to()]))
    }

    fn update<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::UPDATE, self, Some(vec![arg.to()]))
    }

    fn order_by<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::ORDER_BY, self, Some(vec![arg.to()]))
    }

    fn without<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::WITHOUT, self, Some(vec![arg.to()]))
    }

    fn contains<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::CONTAINS, self, Some(vec![arg.to()]))
    }

    fn limit<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::LIMIT, self, Some(vec![arg.to()]))
    }

    fn get<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET, self, Some(vec![arg.to()]))
    }

    fn get_all<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET_ALL, self, Some(vec![arg.to()]))
    }

    fn opt_arg<T>(&self, name: &str, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let mut opt = TA::new();
        opt.set_key(name.into());
        opt.set_val(arg.to());
        let arg = RepeatedField::from_vec(vec![opt]);
        let mut term = self.to();
        term.set_optargs(arg);
        FromTerm::from(term)
    }

    fn insert<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INSERT, self, Some(vec![arg.to()]))
    }

    fn delete(&self) -> Self
        where Self: Sized
    {
        command!(TT::DELETE, self, None as Option<Vec<Term>>)
    }

    fn changes(&self) -> Self
        where Self: Sized
    {
        command!(TT::CHANGES, self, None as Option<Vec<Term>>)
    }

    fn has_fields<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::HAS_FIELDS, self, Some(vec![arg.to()]))
    }

    fn get_field<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET_FIELD, self, Some(vec![arg.to()]))
    }

    fn filter<F>(&self, func: F) -> Self
        where F: FnOnce(Term) -> Term, Self: Sized
    {
        command!(TT::FILTER, self, Some(closure_arg!(func)))
    }

    fn map<F>(&self, func: F) -> Self
        where F: FnOnce(Term) -> Term, Self: Sized
    {
        command!(TT::MAP, self, Some(closure_arg!(func)))
    }

    fn branch<T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let args: Vec<Term> = arg.into_iter()
            .map(|a| a.to())
            .collect();
        command!(TT::BRANCH, self, Some(args))
    }

    fn object<'a, T, O>(&self, arg: T) -> Self
        where O: ToJson + Sized, T: IntoIterator<Item=(&'a str, O)>, Self: Sized
    {
        let mut obj = BTreeMap::new();
        for (key, val) in arg {
            obj.insert(key.to_string(), val.to_json());
        }
        let obj = Value::Object(obj);
        FromTerm::from(obj.to())
    }

    fn array<T, O>(&self, arg: T) -> Self
        where O: ToJson + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let args: Vec<Value> = arg.into_iter()
            .map(|a| a.to_json())
            .collect();
        let arr = Value::Array(args);
        FromTerm::from(arr.to())
    }
}

#[test]
fn test_commands_can_be_chained() {
    impl Command for Term { }
    let r = Term::new();
    let term = r.db("heroes").table("marvel").map(|row| row.get_field("first_appearance"));
    panic!(format!("{:?}\n\n{}", term, term.encode()));
}
