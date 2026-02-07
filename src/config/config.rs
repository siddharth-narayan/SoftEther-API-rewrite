use std::{ffi::c_char, os::raw::c_void, ptr::{self, null_mut, slice_from_raw_parts}, slice::from_raw_parts, str::FromStr};
use toml::{Table, Value};

use crate::{config::structure::Config, mem::structs::list::List, nullcheck, str::{clone_from_c_str, clone_from_uni_str, into_c_str}};

// Either a Folder or a Value
enum GenericItem {
    Folder(Folder),
    Item(Item),
}

pub struct Folder {
    name: *mut c_char,
    items: *mut List<*mut Item>,
    folders: *mut List<*mut Folder>,
    parent: *mut Folder,

    _internal: toml::map::Map<String, GenericItem>,
}

impl Folder {
    pub fn new(table: toml::map::Map<String, Value>) {

    }

    pub fn insert(&mut self, key: String, value: GenericItem) {
        // let item = Item::new(value);
        self._internal.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&GenericItem> {
        self._internal.get(&key)
    }

    pub fn get_mut(&mut self, key: String) -> Option<&mut GenericItem> {
        self._internal.get_mut(&key)
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

fn CfgAdd(folder: *mut Folder, key: *mut c_char, value: Value) -> *mut Item {
    nullcheck!(null_mut(), folder, value);
  
    let folder = unsafe { &mut *folder };
    let key = unsafe { clone_from_c_str(key) };
    
    let item = Item::new(value);
    let item_ptr = (&mut item) as *mut Item;

    let _ = folder.insert(key, GenericItem::Item(item));

    item_ptr
}

fn CfgGet<'a>(folder: *mut Folder, name: *mut c_char) -> Option<&'a mut GenericItem> {
    let folder = unsafe { &mut *folder };
    let name = unsafe { clone_from_c_str(name) };

    folder.get_mut(name)
}

// Just frees?
// void CfgDeleteFolder(FOLDER*f)

// FOLDER *CfgCreateFolder(FOLDER*parent,char*name)

// TOKEN_LIST *CfgEnumFolderToTokenList(FOLDER*f)

// TOKEN_LIST *CfgEnumItemToTokenList(FOLDER*f)

// ITEM *CfgAddInt(FOLDER*f,char*name,UINTi)
pub extern "C" fn CfgAddInt(folder: *mut Folder, name: *mut c_char, i: u32) -> *mut Item {
    nullcheck!(null_mut(), folder, name);

    CfgAdd(folder, name, Value::Integer(i.into()))
}

// ITEM *CfgAddBool(FOLDER*f,char*name,boolb)
pub extern "C" fn CfgAddBool(folder: *mut Folder, name: *mut c_char, b: bool) -> *mut Item {
    nullcheck!(null_mut(), folder, name);

    CfgAdd(folder, name, Value::Boolean(b))
}

// ITEM *CfgAddInt64(FOLDER*f,char*name,UINT64i)
pub extern "C" fn CfgAddInt64(folder: *mut Folder, name: *mut c_char, i: u64) -> *mut Item {
    nullcheck!(null_mut(), folder, name);

    let int: i64 = match i.try_into() {
        Ok(n) => n,
        Err(_) => 0
    };

    CfgAdd(folder, name, Value::Integer(int))
}

// ITEM *CfgAddByte(FOLDER*f,char*name,void*buf,UINTsize)

// ITEM *CfgAddBuf(FOLDER*f,char*name,BUF*b)

// ITEM *CfgAddStr(FOLDER*f,char*name,char*str)
pub extern "C" fn CfgAddStr(folder: *mut Folder, name: *mut c_char, string: *mut c_char) -> *mut Item {
    nullcheck!(null_mut(), folder, name, string);

    let string = unsafe { clone_from_c_str(string) };
    CfgAdd(folder, name, Value::String(string))
}

// ITEM *CfgAddUniStr(FOLDER*f,char*name,wchar_t*str)
pub extern "C" fn CfgAddUniStr(folder: *mut Folder, key: *mut c_char, value: *mut u16) -> *mut Item {
    nullcheck!(null_mut(), folder, key, value);

    let value = toml::Value::String(unsafe { clone_from_uni_str(value) });
    
    CfgAdd(folder, key, value)
}

// FOLDER *CfgGetFolder(FOLDER*parent,char*name)
pub extern "C" fn CfgGetFolder(parent: *mut Folder, name: *mut c_char) -> *mut Folder {
    nullcheck!(null_mut(), parent, name);

    let parent = unsafe { &mut *parent };
    let name = unsafe { clone_from_c_str(name) };

    match parent._internal.get_mut(&name) {
        Some(s) => match s {
            GenericItem::Folder(t) => t,
            _ =>  null_mut(),
        },
        None => {
            null_mut()
        },
    }
}

// UINT CfgGetInt(FOLDER*f,char*name)
pub extern "C" fn CfgGetInt(parent: *mut Folder, name: *mut c_char) -> u32 {
    nullcheck!(0, parent, name);

    let parent = unsafe { &mut *parent };
    let name = unsafe { clone_from_c_str(name) };

    let number = match parent._internal.get_mut(&name) {
        Some(s) => match s {
            GenericItem::Item(t) => match t._internal {
                Value::Integer(i) => i,
                _ => 0,
            },
            _ =>  0,
        },
        None => {
            0
        },
    };

    match u32::try_from(number) {
        Ok(i) => i,
        Err(_) => 0,
    }
}

// bool CfgGetBool(FOLDER*f,char*name)
pub extern "C" fn CfgGetBool(parent: *mut Folder, name: *mut c_char) -> bool {
    nullcheck!(false, parent, name);

    let parent = unsafe { &mut *parent };
    let name = unsafe { clone_from_c_str(name) };

    match parent._internal.get_mut(&name) {
        Some(s) => match s {
            GenericItem::Item(t) => match t._internal {
                Value::Boolean(b) => b,
                _ => false,
            },
            _ =>  false,
        },
        None => {
            false
        },
    }
}

// UINT64 CfgGetInt64(FOLDER*f,char*name)
pub extern "C" fn CfgGetInt64(parent: *mut Folder, name: *mut c_char) -> u64 {
    nullcheck!(0, parent, name);

    let parent = unsafe { &mut *parent };
    let name = unsafe { clone_from_c_str(name) };

    let number = match parent._internal.get_mut(&name) {
        Some(s) => match s {
            GenericItem::Item(t) => match t._internal {
                Value::Integer(b) => b,
                _ => 0,
            },
            _ =>  0,
        },
        None => {
            0
        },
    };

    match u64::try_from(number) {
        Ok(v) => v,
        Err(_) => 0
    }
}

// UINT CfgGetByte(FOLDER*f,char*name,void*buf,UINTsize)
pub extern "C" fn CfgGetBytes(folder: *mut Folder, name: *mut c_char, buf: *mut u8, size: u32) {

}

// BUF *CfgGetBuf(FOLDER*f,char*name)

// bool CfgGetStr(FOLDER*f,char*name,char*str,UINTsize)
pub extern "C" fn CfgGetStr(folder: *mut Folder, name: *mut c_char, dst: *mut u8, size: u32) -> *mut Item {
    nullcheck!(null_mut(), folder, name, dst);

    let item = match CfgGet(folder, name) {
        Some(s) => {
            match s {
                GenericItem::Item(i) => i,
                _ => {
                    return null_mut()
                }
            }
        },
        None => {
            return {
                null_mut()
            }  
        }
    };

    let item_ptr: *mut Item = item;

    let string_bytes = match item._internal {
        Value::String(s) => s.as_bytes_mut(),
        _ => {
            return null_mut()
        }
    };


    let dst= unsafe { std::slice::from_raw_parts_mut(dst, size as usize) };
    dst.copy_from_slice(&string_bytes[0..dst.len()]);

    todo!()
}

// bool CfgGetUniStr(FOLDER*f,char*name,wchar_t*str,UINTsize)

// bool CfgIsItem(FOLDER*f,char*name)
pub extern "C" fn CfgIsItem(folder: *mut Folder, name: *mut c_char) -> bool {
    nullcheck!(false, folder, name);

    match CfgGet(folder, name) {
        Some(s) => {
            match s {
                GenericItem::Folder(_) => false,
                GenericItem::Item(_) => true,
            }
        },
        None => false
    }
}

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


