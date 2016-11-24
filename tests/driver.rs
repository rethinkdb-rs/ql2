extern crate ql2;

use ql2::commands::Command;

pub struct Client;

impl Command for Client { }

#[test]
fn db_works() {
    let r = Client;
    panic!(format!("{:?}", r.db("heroes")));
}
