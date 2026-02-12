use std::ffi::{CString, c_char, c_void};
use std::net::IpAddr;
use std::ptr::null_mut;

use widestring::U16CString;

use crate::{c_export, nullcheck};

use crate::mem::structs::buf::Buffer;
use crate::mem::structs::list::List;
use crate::network::structs::cert::{K, X};
use crate::network::util::IP;
use crate::pack::pack::{Pack, PackElement, PackInnerValue};
use crate::str::clone_from_c_str;
use crate::util::{RawCStr, RawPtr, copy_slice_to_slice};

c_export! {
    // PACK *NewPack()
    fn NewPack() -> *mut Pack {
        Pack::new().as_mut_ptr()
    }

    // void FreePack(PACK*p)
    fn FreePack(ptr: *mut Pack) {
        Pack::free_mut_ptr(ptr);
    }

    // BUF *PackToBuf(PACK*p)
    fn PackToBuf(ptr: *mut Pack) -> *mut Buffer {
        let pack = unsafe { &mut *ptr };

        pack.clone().to_buf().as_mut_ptr()
    }

    // PACK *BufToPack(BUF*b)
    fn BackToPack(ptr: *mut Buffer) -> *mut Pack {
        let buf = unsafe { &mut *ptr };

        Pack::from_buf(buf.clone()).as_mut_ptr()
    }

    // bool PackIsValueExists(PACK*p,char*name)
    // void PackSetCurrentJsonGroupName(PACK*p,char*json_group_name)
    // JSON_VALUE *PackToJson(PACK*p)
    // PACK *JsonToPack(JSON_VALUE*v)

}

// PACK Get implementations
fn PackGetValue<'a>(ptr: *mut Pack, name: *mut c_char) -> Option<&'a mut PackInnerValue>{
    PackGetValueEx(ptr, name, 0)
}

fn PackGetValueEx<'a>(ptr: *mut Pack, name: *mut c_char, index: usize) -> Option<&'a mut PackInnerValue> {
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

// c_export! {
    // ELEMENT *GetElement(PACK*p,char*name,UINTtype)
    fn GetElement(ptr: *mut Pack, name: RawCStr) -> *mut PackElement {
        nullcheck!(null_mut(), ptr, name);

        match PackGetElement(ptr, name) {
            Some(s) => s,
            None => null_mut()
        }
    }

    // UINT PackGetStrSize(PACK*p,char*name)
    fn PackGetStrSize(ptr: *mut Pack, name: RawCStr) -> u32 {
        nullcheck!(0, ptr, name);

        PackGetStrSizeEx(ptr, name, 0)
    }


    // UINT PackGetStrSizeEx(PACK*p,char*name,UINTindex)
    fn PackGetStrSizeEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
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
pub fn PackGetStr(ptr: *mut Pack, name: *mut c_char, str_: *mut u8, size: u32) -> bool {
    nullcheck!(false, ptr, name, str_);

    PackGetStrEx(ptr, name, str_, size, 0)
}

// bool PackGetStrEx(PACK*p,char*name,char*str,UINTsize,UINTindex)
pub fn PackGetStrEx(ptr: *mut Pack, name: *mut c_char, str_: *mut u8, size: u32, index: u32) -> bool {
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
            copy_slice_to_slice(dst_str, src_str.as_bytes_with_nul(), size as usize);
            true
        },
        Err(_) => {
            false
        }
    }
}

// bool PackGetUniStr(PACK*p,char*name,wchar_t*unistr,UINTsize)
pub fn PackGetUniStr(ptr: *mut Pack, name: *mut c_char, unistr: *mut u16, size: u32) -> bool {
    nullcheck!(false, ptr, name, unistr);

    PackGetUniStrEx(ptr, name, unistr, size, 0)
}

// bool PackGetUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTsize,UINTindex)
pub fn PackGetUniStrEx(ptr: *mut Pack, name: *mut c_char, unistr: *mut u16, size: u32, index: u32) -> bool {
    nullcheck!(false, ptr, name, unistr);

    let value = match PackGetValueEx(ptr, name, index as usize) {
        Some(p) => p,
        None => {
            return false;
        }
    };

    let dst_str: &mut [u16] = unsafe { std::slice::from_raw_parts_mut(unistr, size as usize) };
    
    match U16CString::from_str(value.str()) {
        Ok(src_str) => {
            copy_slice_to_slice(dst_str, src_str.as_slice_with_nul(), size as usize);
            true
        },
        Err(_) => {
            false
        }
    }
}

// UINT PackGetIndexCount(PACK*p,char*name)
pub fn PackGetIndexCount(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// UINT PackGetNum(PACK*p,char*name)
// Returns a number <= 65536 for some reason
// Takes up more than 2 bytes anyways?
pub fn PackGetNum(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    u32::min(PackGetInt(ptr, name), 65536)
}

// UINT PackGetInt(PACK*p,char*name)
pub fn PackGetInt(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetIntEx(ptr, name, 0)
}

// UINT PackGetIntEx(PACK*p,char*name,UINTindex)
pub fn PackGetIntEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    let val = match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.u64(),
        None => 0
    };

    match val.try_into() {
        Ok(i) => i,
        Err(_) => 0
    } 
}

// UINT64 PackGetInt64(PACK*p,char*name)
pub fn PackGetInt64(ptr: *mut Pack, name: *mut c_char) -> u64 {
    nullcheck!(0, ptr, name);

    PackGetInt64Ex(ptr, name, 0)
}

// UINT64 PackGetInt64Ex(PACK*p,char*name,UINTindex)
pub fn PackGetInt64Ex(ptr: *mut Pack, name: *mut c_char, index: u32) -> u64 {
    nullcheck!(0, ptr, name);

    let val = match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.u64(),
        None => 0
    };

    match val.try_into() {
        Ok(i) => i,
        Err(_) => 0
    } 
}

// bool PackGetData(PACK*p,char*name,void*data)
pub fn PackGetData(ptr: *mut Pack, name: *mut c_char, data: *mut core::ffi::c_void) -> bool {
    nullcheck!(false, ptr, name, data);

    todo!()
}

// UINT PackGetDataSize(PACK*p,char*name)
pub fn PackGetDataSize(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetDataSizeEx(ptr, name, 0)
}

// UINT PackGetDataSizeEx(PACK*p,char*name,UINTindex)
pub fn PackGetDataSizeEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// BUF *PackGetBuf(PACK*p,char*name)
pub fn PackGetBuf(ptr: *mut Pack, name: *mut c_char) -> *mut Buffer {
    nullcheck!(null_mut(), ptr, name);

    PackGetBufEx(ptr, name, 0)
}

// BUF *PackGetBufEx(PACK*p,char*name,UINTindex)
pub fn PackGetBufEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> *mut Buffer {
    nullcheck!(null_mut(), ptr, name);

    match PackGetValueEx(ptr, name, index as usize) {
        Some(i) => i.buf(),
        None => null_mut()
    }
}

// bool PackGetBool(PACK*p,char*name)
pub fn PackGetBool(ptr: *mut Pack, name: *mut c_char) -> bool {
    nullcheck!(false, ptr, name);

    PackGetBoolEx(ptr, name, 0)
}

// bool PackGetBoolEx(PACK*p,char*name,UINTindex)
pub fn PackGetBoolEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> bool {
    nullcheck!(false, ptr, name);
    
    todo!()
}

// X *PackGetX(PACK*p,char*name)
pub fn PackGetX(ptr: *mut Pack, name: *mut c_char) -> *mut X {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// LIST *PackGetXList(PACK*p,char*name)
pub fn PackGetXList(ptr: *mut Pack, name: *mut c_char) -> *mut List<RawPtr> {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// K *PackGetK(PACK*p,char*name)
pub fn PackGetK(ptr: *mut Pack, name: *mut c_char) -> *mut K {
    nullcheck!(null_mut(), ptr, name);

    todo!()
}

// bool PackGetIp(PACK*p,char*name,IP*ip)
pub fn PackGetIp(ptr: *mut Pack, name: *mut c_char, ip: *mut IP) -> bool {
    nullcheck!(false, ptr, name, ip);

    PackGetIpEx(ptr, name, ip, 0)
}

// bool PackGetIpEx(PACK*p,char*name,IP*ip,UINTindex)
pub fn PackGetIpEx(ptr: *mut Pack, name: *mut c_char, ip: *mut IP, index: u32) -> bool {
    nullcheck!(false, ptr, name, ip);

    todo!()
}

// UINT PackGetIp32(PACK*p,char*name)
pub fn PackGetIp32(ptr: *mut Pack, name: *mut c_char) -> u32 {
    nullcheck!(0, ptr, name);

    PackGetIp32Ex(ptr, name, 0)
}

// UINT PackGetIp32Ex(PACK*p,char*name,UINTindex)
pub fn PackGetIp32Ex(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    nullcheck!(0, ptr, name);

    todo!()
}

// bool PackGetIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex)
pub fn PackGetIp6AddrEx(ptr: *mut Pack, name: *mut c_char, addr: *mut IpAddr, index: u32) -> bool {
    nullcheck!(false, ptr, addr);

    todo!("Use IPV6_ADDR")
}

// bool PackGetData2(PACK*p,char*name,void*data,UINTsize)
pub fn PackGetData2(ptr: *mut Pack, name: *mut c_char, data: *mut c_void, size: u32) -> bool {
    nullcheck!(false, ptr, name, data);

    PackGetDataEx2(ptr, name, data, size, 0)
}

// bool PackGetDataEx2(PACK*p,char*name,void*data,UINTsize,UINTindex)
pub fn PackGetDataEx2(
    ptr: *mut Pack,
    name: *mut c_char,
    data: *mut core::ffi::c_void,
    size: u32,
    index: u32,
) -> bool {
    nullcheck!(false, ptr, name, data);

    todo!()
}

// }


// PACK Add implementations 
// c_export! {
    // ELEMENT *PackAddX(PACK*p,char*name,X*x)
// ELEMENT *PackAddXList(PACK*p,char*name,LIST*chain)
// ELEMENT *PackAddK(PACK*p,char*name,K*k)
// ELEMENT *PackAddStr(PACK*p,char*name,char*str)
// ELEMENT *PackAddStrEx(PACK*p,char*name,char*str,UINTindex,UINTtotal)
// ELEMENT *PackAddUniStr(PACK*p,char*name,wchar_t*unistr)
// ELEMENT *PackAddUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTindex,UINTtotal)
// ELEMENT *PackAddInt(PACK*p,char*name,UINTi)
// ELEMENT *PackAddNum(PACK*p,char*name,UINTnum)
// ELEMENT *PackAddIntEx(PACK*p,char*name,UINTi,UINTindex,UINTtotal)
// ELEMENT *PackAddInt64(PACK*p,char*name,UINT64i)
// ELEMENT *PackAddInt64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
// ELEMENT *PackAddTime64(PACK*p,char*name,UINT64i)
// ELEMENT *PackAddTime64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
// ELEMENT *PackAddData(PACK*p,char*name,void*data,UINTsize)
// ELEMENT *PackAddDataEx(PACK*p,char*name,void*data,UINTsize,UINTindex,UINTtotal)
// ELEMENT *PackAddBuf(PACK*p,char*name,BUF*b)

// ELEMENT *PackAddBool(PACK*p,char*name,boolb)
// ELEMENT *PackAddBoolEx(PACK*p,char*name,boolb,UINTindex,UINTtotal)
// void PackAddIp(PACK*p,char*name,IP*ip)
// void PackAddIpEx(PACK*p,char*name,IP*ip,UINTindex,UINTtotal)

// void PackAddIp32(PACK*p,char*name,UINTip32)
// void PackAddIp32Ex(PACK*p,char*name,UINTip32,UINTindex,UINTtotal)
// ELEMENT *PackAddIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex,UINTtotal)
// }