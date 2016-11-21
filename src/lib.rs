//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde_json;

pub mod proto;

use std::collections::BTreeMap;

use serde_json::value::{Value, Map, ToJson};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
pub use proto::{
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
        let cmd = $cmd.to_term();
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
        FromTerm::from_term(term)
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

pub type Array = Vec<Term>;

pub type Object = Map<String, Term>;

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
    fn to_term(&self) -> Term;
}

pub trait FromTerm {
    fn from_term(t: Term) -> Self;
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
            Value::Array(val) => {
                datum.set_field_type(DT::R_ARRAY);
                let args: Vec<Datum> = val.iter()
                    .map(|a| a.to_term().take_datum())
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
                        obj.set_val(arg.to_term().take_datum());
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
    fn to_term(&self) -> Term {
        self.clone()
    }
}

impl From<BTreeMap<String, Term>> for Term {
    fn from(t: BTreeMap<String, Term>) -> Term {
        // Datum
        let mut datum = Datum::new();
        datum.set_field_type(DT::R_OBJECT);
        let args: Vec<DA> = t.into_iter()
            .map(|(name, mut arg)| {
                let mut obj = DA::new();
                obj.set_key(name.into());
                obj.set_val(arg.take_datum());
                obj
            })
        .collect();
        let obj = RepeatedField::from_vec(args);
        datum.set_r_object(obj);
        // Term
        let mut term = Term::new();
        term.set_field_type(TT::DATUM);
        term.set_datum(datum);
        term
    }
}

impl From<Vec<Term>> for Term {
    fn from(t: Vec<Term>) -> Term {
        // Datum
        let mut datum = Datum::new();
        datum.set_field_type(DT::R_ARRAY);
        let args: Vec<Datum> = t.into_iter()
            .map(|mut a| a.take_datum())
            .collect();
        let arr = RepeatedField::from_vec(args);
        datum.set_r_array(arr);
        // Term
        let mut term = Term::new();
        term.set_field_type(TT::DATUM);
        term.set_datum(datum);
        term
    }
}

impl FromTerm for Term {
    fn from_term(t: Term) -> Term {
        t
    }
}

pub trait Command : FromTerm + ToTerm {
    fn expr<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        FromTerm::from_term(arg.to_term())
    }

    fn db<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB, self, Some(vec![arg.to_term()]))
    }

    fn db_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB_CREATE, self, Some(vec![arg.to_term()]))
    }

    fn db_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::DB_DROP, self, Some(vec![arg.to_term()]))
    }

    fn table<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE, self, Some(vec![arg.to_term()]))
    }

    fn table_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE_CREATE, self, Some(vec![arg.to_term()]))
    }

    fn table_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::TABLE_DROP, self, Some(vec![arg.to_term()]))
    }

    fn index_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INDEX_CREATE, self, Some(vec![arg.to_term()]))
    }

    fn index_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INDEX_DROP, self, Some(vec![arg.to_term()]))
    }

    fn replace<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::REPLACE, self, Some(vec![arg.to_term()]))
    }

    fn update<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::UPDATE, self, Some(vec![arg.to_term()]))
    }

    fn order_by<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::ORDER_BY, self, Some(vec![arg.to_term()]))
    }

    fn without<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::WITHOUT, self, Some(vec![arg.to_term()]))
    }

    fn contains<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::CONTAINS, self, Some(vec![arg.to_term()]))
    }

    fn limit<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::LIMIT, self, Some(vec![arg.to_term()]))
    }

    fn get<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET, self, Some(vec![arg.to_term()]))
    }

    fn get_all<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET_ALL, self, Some(vec![arg.to_term()]))
    }

    fn opt_arg<T>(&self, name: &str, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let mut opt = TA::new();
        opt.set_key(name.into());
        opt.set_val(arg.to_term());
        let arg = RepeatedField::from_vec(vec![opt]);
        let mut term = self.to_term();
        term.set_optargs(arg);
        FromTerm::from_term(term)
    }

    fn insert<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::INSERT, self, Some(vec![arg.to_term()]))
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
        command!(TT::HAS_FIELDS, self, Some(vec![arg.to_term()]))
    }

    fn get_field<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        command!(TT::GET_FIELD, self, Some(vec![arg.to_term()]))
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
            .map(|a| a.to_term())
            .collect();
        command!(TT::BRANCH, self, Some(args))
    }

    fn object<'a, T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=(&'a str, O)>, Self: Sized
    {
        let mut obj = BTreeMap::new();
        for (key, val) in arg {
            obj.insert(key.to_string(), val.to_term());
        }
        FromTerm::from_term(Term::from(obj))
    }

    fn array<T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let arr: Vec<Term> = arg.into_iter()
            .map(|a| a.to_term())
            .collect();
        FromTerm::from_term(Term::from(arr))
    }
}

#[test]
fn test_commands_can_be_chained() {
    impl Command for Term { }
    let r = Term::new();
    //let term = r.db("heroes").table("marvel").map(|row| row.get_field("first_appearance"));
    let term = r.table("marvel").map(|row| row.get_field("first_appearance"));
    panic!(format!("{:?}\n\n{}\n\n{:?}", term, term.encode(), term.info()));
}
