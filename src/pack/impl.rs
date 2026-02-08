use std::ffi::c_char;
use std::net::IpAddr;
use std::ptr::null_mut;

use crate::c_export;

use crate::mem::structs::buf::Buffer;
use crate::mem::structs::list::List;
use crate::network::structs::cert::{K, X};
use crate::network::util::IP;
use crate::pack::pack::{Pack, PackElement, PackInnerValue};
use crate::str::clone_from_c_str;
use crate::util::{RawCStr, RawPtr};

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

        pack.clone().to_buf().as_mut_ptr();
        todo!()
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
        match PackGetElement(ptr, name) {
            Some(s) => s,
            None => null_mut()
        }
    }

    // UINT PackGetStrSize(PACK*p,char*name)
    fn PackGetStrSize(ptr: *mut Pack, name: RawCStr) -> u32 {
        PackGetStrSizeEx(ptr, name, 0)
    }


    // UINT PackGetStrSizeEx(PACK*p,char*name,UINTindex)
    fn PackGetStrSizeEx(ptr: *mut Pack, name: *mut c_char, index: u32) -> u32 {
        let value = match PackGetValueEx(ptr, name, index as usize) {
            Some(p) => p,
            None => {
                return 0;
            }
        };

        value.str().len() as u32
    }

// bool PackGetStr(PACK*p,char*name,char*str,UINTsize)
pub fn PackGetStr(p: *mut Pack, name: *mut c_char, str_: *mut c_char, size: u32) -> bool {
    todo!()
}

// bool PackGetStrEx(PACK*p,char*name,char*str,UINTsize,UINTindex)
pub fn PackGetStrEx(p: *mut Pack, name: *mut c_char, str_: *mut c_char, size: u32, index: u32) -> bool {
    todo!()
}

// bool PackGetUniStr(PACK*p,char*name,wchar_t*unistr,UINTsize)
pub fn PackGetUniStr(p: *mut Pack, name: *mut c_char, unistr: *mut u16, size: u32) -> bool {
    todo!()
}

// bool PackGetUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTsize,UINTindex)
pub fn PackGetUniStrEx(p: *mut Pack, name: *mut c_char, unistr: *mut u16, size: u32, index: u32) -> bool {
    todo!()
}

// UINT PackGetIndexCount(PACK*p,char*name)
pub fn PackGetIndexCount(p: *mut Pack, name: *mut c_char) -> u32 {
    todo!()
}

// UINT PackGetInt(PACK*p,char*name)
pub fn PackGetInt(p: *mut Pack, name: *mut c_char) -> u32 {
    todo!()
}

// UINT PackGetNum(PACK*p,char*name)
pub fn PackGetNum(p: *mut Pack, name: *mut c_char) -> u32 {
    todo!()
}

// UINT PackGetIntEx(PACK*p,char*name,UINTindex)
pub fn PackGetIntEx(p: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    todo!()
}

// UINT64 PackGetInt64(PACK*p,char*name)
pub fn PackGetInt64(p: *mut Pack, name: *mut c_char) -> u64 {
    todo!()
}

// UINT64 PackGetInt64Ex(PACK*p,char*name,UINTindex)
pub fn PackGetInt64Ex(p: *mut Pack, name: *mut c_char, index: u32) -> u64 {
    todo!()
}

// UINT PackGetDataSizeEx(PACK*p,char*name,UINTindex)
pub fn PackGetDataSizeEx(p: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    todo!()
}

// UINT PackGetDataSize(PACK*p,char*name)
pub fn PackGetDataSize(p: *mut Pack, name: *mut c_char) -> u32 {
    todo!()
}

// bool PackGetData(PACK*p,char*name,void*data)
pub fn PackGetData(p: *mut Pack, name: *mut c_char, data: *mut core::ffi::c_void) -> bool {
    todo!()
}

// BUF *PackGetBuf(PACK*p,char*name)
pub fn PackGetBuf(p: *mut Pack, name: *mut c_char) -> *mut Buffer {
    todo!()
}

// BUF *PackGetBufEx(PACK*p,char*name,UINTindex)
pub fn PackGetBufEx(p: *mut Pack, name: *mut c_char, index: u32) -> *mut Buffer {
    todo!()
}

// bool PackGetBool(PACK*p,char*name)
pub fn PackGetBool(p: *mut Pack, name: *mut c_char) -> bool {
    todo!()
}

// bool PackGetBoolEx(PACK*p,char*name,UINTindex)
pub fn PackGetBoolEx(p: *mut Pack, name: *mut c_char, index: u32) -> bool {
    todo!()
}

// X *PackGetX(PACK*p,char*name)
pub fn PackGetX(p: *mut Pack, name: *mut c_char) -> *mut X {
    todo!()
}

// LIST *PackGetXList(PACK*p,char*name)
pub fn PackGetXList(p: *mut Pack, name: *mut c_char) -> *mut List<RawPtr> {
    todo!()
}

// K *PackGetK(PACK*p,char*name)
pub fn PackGetK(p: *mut Pack, name: *mut c_char) -> *mut K {
    todo!()
}

// bool PackGetIp(PACK*p,char*name,IP*ip)
pub fn PackGetIp(p: *mut Pack, name: *mut c_char, ip: *mut IP) -> bool {
    todo!()
}

// bool PackGetIpEx(PACK*p,char*name,IP*ip,UINTindex)
pub fn PackGetIpEx(p: *mut Pack, name: *mut c_char, ip: *mut IP, index: u32) -> bool {
    todo!()
}

// UINT PackGetIp32(PACK*p,char*name)
pub fn PackGetIp32(p: *mut Pack, name: *mut c_char) -> u32 {
    todo!()
}

// UINT PackGetIp32Ex(PACK*p,char*name,UINTindex)
pub fn PackGetIp32Ex(p: *mut Pack, name: *mut c_char, index: u32) -> u32 {
    todo!()
}

// bool PackGetIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex)
pub fn PackGetIp6AddrEx(p: *mut Pack, name: *mut c_char, addr: *mut IpAddr, index: u32) -> bool {
    todo!("Use IPV6_ADDR")
}

// bool PackGetData2(PACK*p,char*name,void*data,UINTsize)
pub fn PackGetData2(p: *mut Pack, name: *mut c_char, data: *mut core::ffi::c_void, size: u32) -> bool {
    todo!()
}

// bool PackGetDataEx2(PACK*p,char*name,void*data,UINTsize,UINTindex)
pub fn PackGetDataEx2(
    p: *mut Pack,
    name: *mut c_char,
    data: *mut core::ffi::c_void,
    size: u32,
    index: u32,
) -> bool {
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