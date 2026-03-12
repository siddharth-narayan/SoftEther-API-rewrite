use std::ffi::{CString, c_char, c_void};
use std::net::IpAddr;
use std::ptr::null_mut;
use std::slice;

use widestring::U16CString;

use crate::{c_export, nullcheck, util};

use crate::mem::structs::buf::Buffer;
use crate::mem::structs::list::List;
use crate::network::structs::cert::{K, X};
use crate::network::util::IP;
use crate::pack::pack::{Pack, PackElement, PackInnerValue};
use crate::str::clone_from_c_str;
use crate::util::{RawCStr, RawPtr, copy};

// PACK *NewPack()
#[unsafe(no_mangle)]
pub extern "C" fn NewPack() -> *mut Pack {
    Pack::new().as_mut_ptr()
}

// void FreePack(PACK*p)
#[unsafe(no_mangle)]
pub extern "C" fn FreePack(ptr: *mut Pack) {
    Pack::free_mut_ptr(ptr);
}

// BUF *PackToBuf(PACK*p)
#[unsafe(no_mangle)]
pub extern "C" fn PackToBuf(ptr: *mut Pack) -> *mut Buffer {
    let pack = unsafe { &mut *ptr };

    pack.clone().to_buf().as_mut_ptr()
}

// PACK *BufToPack(BUF*b)
#[unsafe(no_mangle)]
pub extern "C" fn BackToPack(ptr: *mut Buffer) -> *mut Pack {
    let buf = unsafe { &mut *ptr };

    Pack::from_buf(buf.clone()).as_mut_ptr()
}

// bool PackIsValueExists(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackIsValueExists(pack: *mut Pack, name: RawCStr) {

}

// void PackSetCurrentJsonGroupName(PACK*p,char*json_group_name)#[unsafe(no_mangle)]
pub extern "C" fn PackSetCurrentJsonGroupName(pack: *mut Pack, name: RawCStr) {
    
}

// JSON_VALUE *PackToJson(PACK*p)
#[unsafe(no_mangle)]
pub extern "C" fn PackToJson(pack: *mut Pack, name: RawCStr) {
    
}
// PACK *JsonToPack(JSON_VALUE*v)
#[unsafe(no_mangle)]
pub extern "C" fn JsonToPack(pack: *mut Pack, name: RawCStr) {
    
}
