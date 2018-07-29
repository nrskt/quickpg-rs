extern crate postgres;
extern crate quickpg;

use postgres::{Connection, TlsMode};
use quickpg::model::TableList;

fn main() {
    let conn =
        Connection::connect("postgresql://postgres@localhost:5432/test", TlsMode::None).unwrap();
    let list = TableList::new(&conn);
    println!("{:?}", list)
}
