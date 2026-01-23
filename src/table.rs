// GetTableNameStartWith(char* str)
// GetTableStr(char* name)
// GetTableUniStr(char* name)
// GetUniErrorStr(UINT err)
// GetTableInt(char* name)
// GetCurrentLangId()

use std::{collections::HashMap, ffi::{c_char, c_schar, c_ushort}, fs::{File, read_to_string}};

#[repr(C)]
struct TOKEN_LIST {
    token_count: u32,
    tokens: *const *const c_char
}

struct Table {
    _inner: HashMap<String, String>
}

impl Table {
    pub fn new() -> Self {
        Table { _inner: HashMap::new() }
    }

    pub fn add(&mut self, k: String, v: String) {
        self._inner.insert(k, v);
    }
}
// This function loads a "table" which is a set of strings for debugging and showing to the user
pub fn load_table() -> Option<Table> {
    let mut table = Table::new();
    let content = read_to_string("strtable_en.stb").ok()?;
    
    for mut line in content.lines() {
        line = line.trim_start();
        
        if let Some(comment_idx) = line.find("#") {
            line = &line[..comment_idx]
        }

        if let Some((id, value)) = line.split_once(" ") {
            table.add(String::from(id), String::from(value));
        }
    }

    Some(table)
}

pub extern "C" fn GetTableNameStartWith(str: *const c_char) {

}

// Mayaqua internal?
pub extern "C" fn GetTableStr(name: *const c_char) {

}

pub extern "C" fn GetTableUniStr(name: *const c_char) {

}

pub extern "C" fn GetUniErrorStr(err: u32) -> *const c_ushort {

}

pub extern "C" fn GetTableInt(name: *const c_char) -> u32 {
    return 0;
}

pub extern "C" fn GetCurrentLangId() -> u32 {
    return 1; // 0 for Japanese, 1 for English
}