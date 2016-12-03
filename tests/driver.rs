extern crate ql2;

use ql2::commands::*;
use ql2::types;
/*
use ql2::commands::ReadMode::Outdated;
use ql2::commands::IdentifierFormat::Uuid;
*/
#[test]
fn db_works() {
    let query = r.db("heroes")
        .table("marvel")
        .map(|row: Arg| {
            //(row as Command<types::Object, ()>).get_field::<_, types::Object>("villain")
            row.get_field::<_, types::Object>("villain")
        })
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
