//! RethinkDB protocol implementation in Rust

extern crate protobuf;
extern crate serde_json;

pub mod proto;

use serde_json::value::{Value, ToJson};
use protobuf::repeated::RepeatedField;
use protobuf::ProtobufEnum;
use proto::{
    Term, Datum,
    Datum_DatumType as DT,
    Term_TermType as TT,
};

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
                unimplemented!();
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
    fn to(&self) -> Term {
        self.clone()
    }
}

impl FromTerm for Term {
    fn from(t: Term) -> Term {
        t
    }
}

macro_rules! command {
    ($T:expr, $cmd:expr, $args:expr, $opts:expr) => {{
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
        /*
           if let Some(_opt) = $opts {
           unimplemented!();
           }
           */
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
        FromTerm::from(var)
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
        let args = RepeatedField::from_vec(vec![datum, $res.to()]);
        func.set_args(args);
        func
    }}
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
        let term = arg.to();
        command!(TT::DB, self, Some(vec![term]), None)
    }

    fn db_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::DB_CREATE, self, Some(vec![term]), None)
    }

    fn db_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::DB_DROP, self, Some(vec![term]), None)
    }

    fn table<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::TABLE, self, Some(vec![term]), None)
    }

    fn table_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::TABLE_CREATE, self, Some(vec![term]), None)
    }

    fn table_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::TABLE_DROP, self, Some(vec![term]), None)
    }

    fn index_create<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::INDEX_CREATE, self, Some(vec![term]), None)
    }

    fn index_drop<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::INDEX_DROP, self, Some(vec![term]), None)
    }

    fn replace<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::REPLACE, self, Some(vec![term]), None)
    }

    fn update<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::UPDATE, self, Some(vec![term]), None)
    }

    fn order_by<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::ORDER_BY, self, Some(vec![term]), None)
    }

    fn without<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::WITHOUT, self, Some(vec![term]), None)
    }

    fn contains<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::CONTAINS, self, Some(vec![term]), None)
    }

    fn limit<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::LIMIT, self, Some(vec![term]), None)
    }

    fn get<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::GET, self, Some(vec![term]), None)
    }

    fn get_all<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::GET_ALL, self, Some(vec![term]), None)
    }

    fn insert<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::INSERT, self, Some(vec![term]), None)
    }

    fn delete(&self) -> Self
        where Self: Sized
    {
        command!(TT::DELETE, self, None as Option<Vec<Term>>, None)
    }

    fn changes(&self) -> Self
        where Self: Sized
    {
        command!(TT::CHANGES, self, None as Option<Vec<Term>>, None)
    }

    fn has_fields<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::HAS_FIELDS, self, Some(vec![term]), None)
    }

    fn get_field<T>(&self, arg: T) -> Self
        where T: ToTerm, Self: Sized
    {
        let term = arg.to();
        command!(TT::GET_FIELD, self, Some(vec![term]), None)
    }

    fn filter<F>(&self, func: F) -> Self
        where F: Fn(Self) -> Self, Self: Sized
    {
        let res = func(closure_par!());
        let term = closure_arg!(res);
        command!(TT::FILTER, self, Some(vec![term]), None)
    }

    fn map<F>(&self, func: F) -> Self
        where F: Fn(Self) -> Self, Self: Sized
    {
        let res = func(closure_par!());
        let term = closure_arg!(res);
        command!(TT::MAP, self, Some(vec![term]), None)
    }

    fn branch<T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let args: Vec<Term> = arg.into_iter()
            .map(|a| a.to())
            .collect();
        command!(TT::BRANCH, self, Some(args), None)
    }

    fn object<T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let args: Vec<Term> = arg.into_iter()
            .map(|a| a.to())
            .collect();
        command!(TT::MAKE_OBJ, self, Some(args), None)
    }

    fn array<T, O>(&self, arg: T) -> Self
        where O: ToTerm + Sized, T: IntoIterator<Item=O>, Self: Sized
    {
        let args: Vec<Term> = arg.into_iter()
            .map(|a| a.to())
            .collect();
        command!(TT::MAKE_ARRAY, self, Some(args), None)
    }
}

#[test]
fn test_commands_can_be_chained() {
    impl Command for Term { }
    let r = Term::new();
    let term = r.db("heroes").table("marvel").map(|row| row.get_field("first_appearance"));
    panic!(format!("{:?}\n\n{}", term, term.encode()));
}
