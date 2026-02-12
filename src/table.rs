use std::{
    cell::LazyCell,
    collections::HashMap,
    ffi::{CStr, CString, OsStr, c_char, c_schar, c_ushort},
    fs::{File, read_to_string},
    ptr::{null, null_mut},
    sync::LazyLock,
};

pub static TABLE: LazyLock<Table> = LazyLock::new(load_table);

#[repr(C)]
struct TOKEN_LIST {
    token_count: u32,
    tokens: *const *const c_char,
}

pub struct Table {
    _inner: HashMap<String, String>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            _inner: HashMap::new(),
        }
    }

    pub fn print(&self) {
        for (key, value) in self._inner.iter() {
            println!("{:40} {}", key, value)
        }
    }

    pub fn size(&self) -> usize {
        self._inner.len()
    }

    pub fn add(&mut self, k: String, v: String) {
        self._inner.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self._inner.get(k)
    }
}

// This function loads a "table" which is a set of strings for debugging and showing to the user
pub fn load_table() -> Table {
    let mut table = Table::new();
    let content = read_to_string("strtable_en.stb")
        .ok()
        .unwrap_or(String::new());

    for mut line in content.lines() {
        line = line.trim_start();

        if let Some(comment_idx) = line.find("#") {
            line = &line[..comment_idx]
        }

        if let Some((id, value)) = line.split_once(char::is_whitespace) {
            let value = value.trim_start(); // Untrimed from the whitespace split
            table.add(String::from(id), String::from(value));
        }
    }

    table
}

// TOKEN_LIST *GetTableNameStartWith(char*str)
pub extern "C" fn GetTableNameStartWith(str: *const c_char) {
    todo!()
}

// Mayaqua internal?
// char *GetTableStr(char*name)
pub extern "C" fn GetTableStr(name: *const c_char) -> *const u8 {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().unwrap_or("");

    if let Some(value) = TABLE.get(name) {
        value.as_ptr()
    } else {
        null()
    }
}

// wchar_t *GetTableUniStr(char*name)
pub extern "C" fn GetTableUniStr(name: *const c_char) -> *const c_ushort {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().unwrap_or("");

    if let Some(value) = TABLE.get(name) {
        let new = value.clone();
        let str = new.as_str();
        let vec: Vec<u16> = str.encode_utf16().chain(Some(0)).collect();
        vec.as_ptr() // TODO: vec dropped after this call -- unsafe
    } else {
        null()
    }
}

// wchar_t *GetUniErrorStr(UINTerr)
pub extern "C" fn GetUniErrorStr(err: u32) -> *const c_ushort {
    let lookup = format!("ERR_{}", err);
    if let Some(value) = TABLE.get(&lookup) {
        let new = value.clone();
        let str = new.as_str();
        let vec: Vec<u16> = str.encode_utf16().chain(Some(0)).collect();
        vec.as_ptr() // TODO: vec dropped after this call -- unsafe
    } else {
        null_mut()
    }
}

// UINT GetTableInt(char*name)
pub extern "C" fn GetTableInt(name: *const c_char) -> u32 {
    return 0;
}

// UINT GetCurrentLangId()
pub extern "C" fn GetCurrentLangId() -> u32 {
    return 1; // 0 for Japanese, 1 for English
}