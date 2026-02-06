use std::{ffi::c_char, os::raw::c_void, ptr::null_mut, str::FromStr};
use toml::{Table, Value};

use crate::{config::structure::Config, mem::structs::list::List, nullcheck, str::{clone_from_c_str, clone_from_uni_str}};

pub struct Folder {
    name: *mut c_char,
    items: *mut List<*mut Item>,
    folders: *mut List<*mut Folder>,
    parent: *mut Folder,

    _internal: toml::map::Map<String, Item>,
}

impl Folder {
    pub fn insert(&mut self, key: String, value: Item) {
        // let item = Item::new(value);
        self._internal.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&Item> {
        self._internal.get(&key)
    }
}

pub struct Item {
    name: *mut c_char,
    _type: u32,
    buf: *mut c_void,
    size: u32,
    parent: *const Folder,

    _internal: Value,
}

impl Item {
    // Construct from an existing value
    pub fn new(v: Value) -> Self {
        Self {
            name: null_mut(),
            _type: 0,
            buf: null_mut(),
            size: 0,
            parent: null_mut(),

            _internal: v,
        }
    }

    pub fn into_mut_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Self) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn into_folder(self) -> Option<Folder> {
        let table = match self._internal {
            Value::Table(t) => t,
            _ => {
                return None
            },
        };

        // Folder::new(table);

        todo!()
    }
}


// SoftEther "Folder"is toml Table
pub fn get() {
    let str = r#"
        key = "vaaaaalue"
        [section]
        a = "b"
    "#;

    let c = match Table::from_str(str) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse TOML: {}", e);
            return
        }
    };

    println!("{}", c.get("key").unwrap().as_str().unwrap())
    // Prints "vaaaaalue"
}

// Just frees?
// void CfgDeleteFolder(FOLDER*f)

// FOLDER *CfgCreateFolder(FOLDER*parent,char*name)

// TOKEN_LIST *CfgEnumFolderToTokenList(FOLDER*f)

// TOKEN_LIST *CfgEnumItemToTokenList(FOLDER*f)

// ITEM *CfgAddInt(FOLDER*f,char*name,UINTi)

// ITEM *CfgAddBool(FOLDER*f,char*name,boolb)

// ITEM *CfgAddInt64(FOLDER*f,char*name,UINT64i)

// ITEM *CfgAddByte(FOLDER*f,char*name,void*buf,UINTsize)

// ITEM *CfgAddBuf(FOLDER*f,char*name,BUF*b)

// ITEM *CfgAddStr(FOLDER*f,char*name,char*str)

// ITEM *CfgAddUniStr(FOLDER*f,char*name,wchar_t*str)
pub extern "C" fn CfgAddUniStr(f: *mut Folder, key: *mut c_char, value: *mut u16) -> *mut Item {
    nullcheck!(null_mut(), f, key, value);

    let folder = unsafe { &mut *f };
    let key = unsafe { clone_from_c_str(key) };
    let value = toml::Value::String(unsafe { clone_from_uni_str(value) });
    
    let item = Item::new(value);

    let _ = folder.insert(key, item);

    todo!()
}

// FOLDER *CfgGetFolder(FOLDER*parent,char*name)
pub extern "C" fn CfgGetFolder(parent: *mut Folder, name: *mut c_char) -> *mut Folder {
    nullcheck!(null_mut(), parent, name);

    let parent = unsafe { &mut *parent };
    let name = unsafe { clone_from_c_str(name) };

    let table = match parent._internal.get(&name) {
        Some(s) => match &s._internal {
            Value::Table(t) => t,
            _ => {
                return null_mut();
            }
        },
        None => {
            return null_mut();
        },
    };


    
    todo!()

}

// UINT CfgGetInt(FOLDER*f,char*name)


// bool CfgGetBool(FOLDER*f,char*name)

// UINT64 CfgGetInt64(FOLDER*f,char*name)

// UINT CfgGetByte(FOLDER*f,char*name,void*buf,UINTsize)

// BUF *CfgGetBuf(FOLDER*f,char*name)

// bool CfgGetStr(FOLDER*f,char*name,char*str,UINTsize)

// bool CfgGetUniStr(FOLDER*f,char*name,wchar_t*str,UINTsize)

// bool CfgIsItem(FOLDER*f,char*name)

// BUF *CfgFolderToBuf(FOLDER*f,booltextmode)

// BUF *CfgFolderToBufEx(FOLDER*f,booltextmode,boolno_banner)

// FOLDER *CfgBufTextToFolder(BUF*b)

// char *CfgReadNextLine(BUF*b)

// CFG_RW *NewCfgRw(FOLDER**root,char*cfg_name)

// CFG_RW *NewCfgRwEx2A(FOLDER**root,char*cfg_name_a,booldont_backup,char*template_name_a)

// UINT SaveCfgRw(CFG_RW*rw,FOLDER*f)

// UINT SaveCfgRwEx(CFG_RW*rw,FOLDER*f,UINTrevision_number)

// void FreeCfgRw(CFG_RW*rw)

// ITEM *CfgAddIp32(FOLDER*f,char*name,UINTip)

// UINT CfgGetIp32(FOLDER*f,char*name)

// bool CfgGetIp6Addr(FOLDER*f,char*name,IPV6_ADDR*addr)

// ITEM *CfgAddIp6Addr(FOLDER*f,char*name,IPV6_ADDR*addr)

// bool CfgGetIp(FOLDER*f,char*name,structIP*ip)

// ITEM *CfgAddIp(FOLDER*f,char*name,structIP*ip)


