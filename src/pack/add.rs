use std::{ffi::c_char, ptr::null_mut};

use crate::{network::structs::cert::X, pack::pack::{Pack, PackElement}};

// ELEMENT *PackAddX(PACK*p,char*name,X*x)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddX(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddXList(PACK*p,char*name,LIST*chain)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddXList(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddK(PACK*p,char*name,K*k)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddK(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddStr(PACK*p,char*name,char*str)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddStr(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddStrEx(PACK*p,char*name,char*str,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddStrEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddUniStr(PACK*p,char*name,wchar_t*unistr)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddUniStr(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddUniStrEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddInt(PACK*p,char*name,UINTi)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddInt(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddNum(PACK*p,char*name,UINTnum)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddNum(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddIntEx(PACK*p,char*name,UINTi,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIntEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddInt64(PACK*p,char*name,UINT64i)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddInt64(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddInt64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddInt64Ex(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddTime64(PACK*p,char*name,UINT64i)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddTime64(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddTime64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddTime64Ex(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddData(PACK*p,char*name,void*data,UINTsize)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddData(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddDataEx(PACK*p,char*name,void*data,UINTsize,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddDataEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddBuf(PACK*p,char*name,BUF*b)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddBuf(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddBool(PACK*p,char*name,boolb)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddBool(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddBoolEx(PACK*p,char*name,boolb,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddBoolEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// void PackAddIp(PACK*p,char*name,IP*ip)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIp(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// void PackAddIpEx(PACK*p,char*name,IP*ip,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIpEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// void PackAddIp32(PACK*p,char*name,UINTip32)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIp32(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// void PackAddIp32Ex(PACK*p,char*name,UINTip32,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIp32Ex(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}

// ELEMENT *PackAddIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex,UINTtotal)
#[unsafe(no_mangle)]
pub extern "C" fn PackAddIp6AddrEx(pack: *mut Pack, name: *mut c_char, x: *mut X) -> *mut PackElement {
    null_mut()
}
