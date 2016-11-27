extern crate ql2;

use ql2::commands::*;
use ql2::types::ReadMode::Outdated;
use ql2::types::IdentifierFormat::Uuid;

pub struct Client;

impl Command for Client { }

#[test]
fn db_works() {
    let r = Client;

    let tbl = r.db("heroes").table("marvel")
        .read_mode(Outdated)
        .identifier_format(Uuid);

    let query = tbl.changes()
        .squash(6.9);

    panic!(format!("{:?}", query));
}
