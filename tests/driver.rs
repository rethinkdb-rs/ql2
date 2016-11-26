extern crate ql2;

use ql2::commands::*;

pub struct Client;

impl Command for Client { }

#[test]
fn db_works() {
    let r = Client;
    let query = r.db("heroes").table("marvel");
    panic!(format!("{:?}", query.changes()));
}
