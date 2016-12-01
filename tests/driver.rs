extern crate ql2;

use ql2::commands::*;
//use ql2::types;
use ql2::commands::ReadMode::Outdated;
use ql2::commands::IdentifierFormat::Uuid;

#[test]
fn db_works() {
    let query = r.db("heroes")
        .table("marvel").read_mode(Outdated).identifier_format(Uuid)
        .get_all("spiderman").index("Nickname")
        .changes()
        //.get_field("name")
        ;
    /*
    */

    /*
    let query = r.map("John Doe", |seq| {
        //seq.info()
    });
    */

    panic!(format!("{:?}", query));
}
