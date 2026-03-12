use core::slice;
use std::{ffi::{CString, c_char, c_void}, net::IpAddr, ptr::null_mut};

use widestring::U16CString;

use crate::{mem::structs::{buf::Buffer, list::List}, network::{structs::cert::{K, X}, util::IP}, nullcheck, pack::pack::{Pack, PackElement, PackInnerValue}, str::clone_from_c_str, util::{self, RawCStr, RawPtr, copy}};

// PACK Get implementations
fn PackGetValue<'a>(ptr: *mut Pack, name: *mut c_char) -> Option<&'a mut PackInnerValue> {
    PackGetValueEx(ptr, name, 0)
}

fn PackGetValueEx<'a>(
    ptr: *mut Pack,
    name: *mut c_char,
    index: usize,
) -> Option<&'a mut PackInnerValue> {
    let pack = unsafe { &mut *ptr };
    let name = unsafe { clone_from_c_str(name) };

    let element = match pack.get_element(name) {
        Some(s) => s,
        None => {
            return None;
        }
    };

    element.get_mut(index)
}

fn PackGetElement<'a>(ptr: *mut Pack, name: *mut c_char) -> Option<&'a mut PackElement> {
    let pack = unsafe { &mut *ptr };
    let name = unsafe { clone_from_c_str(name) };

    pack.get_element(name)
}

// ELEMENT *GetElement(PACK*p,char*name,UINTtype)
#[unsafe(no_mangle)]
pub extern "C" fn GetElement(ptr: *mut Pack, name: RawCStr) -> *mut PackElement {
    nullcheck!(null_mut(), ptr, name);

    match PackGetElement(ptr, name) {
        Some(s) => s,
        None => null_mut(),
    }
}

// UINT PackGetStrSize(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetStrSize(ptr: *mut Pack, name: RawCStr) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetStrSizeEx(ptr, name, 0)
}

// UINT PackGetStrSizeEx(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetStrSizeEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    let value = match PackGetValueEx(ptr, name, index as usize) {
        Some(p) => p,
        None => {
            return 0;
        }
    };

    value.str().len() as u32
}

// bool PackGetStr(PACK*p,char*name,char*str,UINTsize)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetStr(ptr: *mut Pack, name: *mut c_char, str_: *mut u8, size: u32) -> bool {
    nullcheck!(false, ptr, name, str_);

    PackGetStrEx(ptr, name, str_, size, 0)
}

// bool PackGetStrEx(PACK*p,char*name,char*str,UINTsize,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetStrEx(
    ptr: *mut Pack,
    name: *mut c_char,
    str_: *mut u8,
    size: u32,
    index: u32,
) -> bool {
    nullcheck!(false, ptr, name, str_);

    let value = match PackGetValueEx(ptr, name, index as usize) {
        Some(p) => p,
        None => {
            return false;
        }
    };

    let dst_str: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(str_, size as usize) };

    match CString::new(value.str()) {
        Ok(src_str) => {
            copy(dst_str, src_str.as_bytes_with_nul());
            true
        }
        Err(_) => false,
    }
}

// bool PackGetUniStr(PACK*p,char*name,wchar_t*unistr,UINTsize)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetUniStr(ptr: *mut Pack, name: *mut c_char, unistr: *mut u16, size: u32) -> bool {
    nullcheck!(false, ptr, name, unistr);

    PackGetUniStrEx(ptr, name, unistr, size, 0)
}

// bool PackGetUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTsize,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetUniStrEx(
    ptr: *mut Pack,
    name: *mut c_char,
    unistr: *mut u16,
    size: u32,
    index: u32,
) -> bool {
    nullcheck!(false, ptr, name, unistr);

    let value = match PackGetValueEx(ptr, name, index as usize) {
        Some(p) => p,
        None => {
            return false;
        }
    };

    let dst_str = unsafe { std::slice::from_raw_parts_mut(unistr, size as usize) };

    match U16CString::from_str(value.str()) {
        Ok(src_str) => {
            copy(dst_str, src_str.as_slice_with_nul());
            true
        }
        Err(_) => false,
    }
}

// UINT PackGetIndexCount(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIndexCount(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// UINT PackGetNum(PACK*p,char*name)
// Returns a number <= 65536 for some reason
// Takes up more than 2 bytes anyways?
#[unsafe(no_mangle)]
pub extern "C" fn PackGetNum(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    u32::min(PackGetInt(ptr, name), 65536)
}

// UINT PackGetInt(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetInt(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetIntEx(ptr, name, 0)
}

// UINT PackGetIntEx(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIntEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    let val = match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.u64(),
        None => 0,
    };

    match val.try_into() {
        Ok(i) => i,
        Err(_) => 0,
    }
}

// UINT64 PackGetInt64(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetInt64(ptr: *mut Pack, name: *mut c_char) -> u64 {
    nullcheck!(0, ptr, name);

    PackGetInt64Ex(ptr, name, 0)
}

// UINT64 PackGetInt64Ex(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetInt64Ex(ptr: *mut Pack, name: *mut c_char, index: u32) -> u64 {
    nullcheck!(0, ptr, name);

    let val = match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.u64(),
        None => 0,
    };

    match val.try_into() {
        Ok(i) => i,
        Err(_) => 0,
    }
}

// bool PackGetData(PACK*p,char*name,void*data)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetData(ptr: *mut Pack, name: *mut c_char, data: *mut core::ffi::c_void) -> bool {
    nullcheck!(false, ptr, name, data);

    todo!()
}

// UINT PackGetDataSize(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetDataSize(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetDataSizeEx(ptr, name, 0)
}

// UINT PackGetDataSizeEx(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetDataSizeEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// BUF *PackGetBuf(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetBuf(ptr: *mut Pack, name: *mut c_char) -> *mut Buffer {
    nullcheck!(null_mut(), ptr, name);

    PackGetBufEx(ptr, name, 0)
}

// BUF *PackGetBufEx(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetBufEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> *mut Buffer {
    nullcheck!(null_mut(), ptr, name);

    match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.buf(),
        None => null_mut(),
    }
}

// bool PackGetBool(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetBool(ptr: *mut Pack, name: *mut c_char) -> bool {
    nullcheck!(false, ptr, name);

    PackGetBoolEx(ptr, name, 0)
}

// bool PackGetBoolEx(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetBoolEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> bool {
    nullcheck!(false, ptr, name);

    todo!()
}

// X *PackGetX(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetX(ptr: *mut Pack, name: *mut c_char) -> *mut X {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// LIST *PackGetXList(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetXList(ptr: *mut Pack, name: *mut c_char) -> *mut List<RawPtr> {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// K *PackGetK(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetK(ptr: *mut Pack, name: *mut c_char) -> *mut K {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// bool PackGetIp(PACK*p,char*name,IP*ip)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIp(ptr: *mut Pack, name: *mut c_char, ip: *mut IP) -> bool {
    nullcheck!(false, ptr, name, ip);

    PackGetIpEx(ptr, name, ip, 0)
}

// bool PackGetIpEx(PACK*p,char*name,IP*ip,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIpEx(ptr: *mut Pack, name: *mut c_char, ip: *mut IP, index: u32) -> bool {
    nullcheck!(false, ptr, name, ip);

    todo!()
}

// UINT PackGetIp32(PACK*p,char*name)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIp32(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetIp32Ex(ptr, name, 0)
}

// UINT PackGetIp32Ex(PACK*p,char*name,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIp32Ex(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// bool PackGetIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetIp6AddrEx(ptr: *mut Pack, name: *mut c_char, addr: *mut IpAddr, index: u32) -> bool {
    nullcheck!(false, ptr, addr);

    todo!("Use IPV6_ADDR")
}

// bool PackGetData2(PACK*p,char*name,void*data,UINTsize)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetData2(ptr: *mut Pack, name: *mut c_char, data: *mut c_void, size: u32) -> bool {
    nullcheck!(false, ptr, name, data);

    PackGetDataEx2(ptr, name, data, size, 0)
}

// bool PackGetDataEx2(PACK*p,char*name,void*data,UINTsize,UINTindex)
#[unsafe(no_mangle)]
pub extern "C" fn PackGetDataEx2(
    ptr: *mut Pack,
    name: *mut c_char,
    data: *mut c_void,
    size: u32,
    index: u32,
) -> bool {
    let data = data as *mut u8;

    nullcheck!(false, ptr, name, data);

    let value = match PackGetValueEx(ptr, name, index as usize) {
        Some(v) => v,
        None => {
            return false;
        }
    };

    let src = value.buf().as_slice();
    let data = unsafe { slice::from_raw_parts_mut(data, size as usize) };
    util::copy(data, src);

    true
}
