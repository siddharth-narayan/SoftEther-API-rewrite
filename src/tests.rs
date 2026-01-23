use crate::table::{self, load_table};


#[test]
fn my_test() {
    let table = load_table();

    assert!(table.is_some());

    let table = table.unwrap();
    table.print();
}
