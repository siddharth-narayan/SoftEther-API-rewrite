use std::ffi::c_void;

use crate::{mem::structs::list::FfiCompareFunction, table::{self, load_table}};


#[test]
fn table_loaded() {
    let table_len = table::TABLE.size();
    assert!(table_len > 100);

    println!("strtable_en.stb loaded with {} entries", table_len);
}

// To check we can transmute somewhat safely
#[test]
fn ptr_cast_to_fn_ok() {
    assert_eq!(
    size_of::<Option<FfiCompareFunction>>(),
    size_of::<*const c_void>()
);
}