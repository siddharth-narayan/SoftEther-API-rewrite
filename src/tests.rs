use crate::table::{self, load_table};


#[test]
fn table_loaded() {
    let table_len = table::TABLE.size();
    assert!(table_len > 100);

    println!("strtable_en.stb loaded with {} entries", table_len);
}
