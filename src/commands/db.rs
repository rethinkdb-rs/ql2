//! ReQL command: db
//!
//! ## Command syntax
//!
//! > r.db(dbName) → db
//!
//! ## Description
//!
//! Reference a database.
//!
//! The `db` command is optional. If it is not present in a query, the query will run against the
//! default database for the connection, specified in the `db` argument to [connect](../connect/index.html).
//!
//! ## Example
//!
//! Explicitly specify a database for a query.
//!
//! ```norun
//! r.db("heroes").table("marvel").run();
//! ```
//!
//! ## Related commands
//!
//! * [table](../table/index.html)
//! * [db_list](../db_list/index.html)

#![allow(dead_code)]

use types;
use types::Command as Cmd;
use proto::Term_TermType as TermType;
use super::Command;

impl Command<(), ()> {
    /// Reference a database. [Read more](db/index.html)
    pub fn db<T>(&self, arg: T) -> Command<types::Db, ()>
        where T: Into<types::String>
    {
        Cmd::make(TermType::DB, Some(vec![arg.into()]), None, Root!())
    }
}
