// UINT DnsThreadNum()
// UINT DnsThreadNumMax()
// void DnsThreadNumMaxSet(constUINTnum)
// bool DnsResolve(IP*ipv6,IP*ipv4,constchar*hostname,UINTtimeout,volatileconstbool*cancel_flag)
// bool GetIPEx(IP*ip,constchar*hostname,UINTtimeout,volatileconstbool*cancel_flag)

use std::{ffi::CStr, net::ToSocketAddrs};

use crate::network::IP;

pub extern "C" fn DnsThreadNum() -> u32 {
    return 1;
}

pub extern "C" fn DnsThreadNumMax() -> u32 {
    return 8;
}

pub extern "C" fn DnsThreadNumMaxSet(num: u32) {

}

pub extern "C" fn DnsResolve(ipv6: *mut IP, ipv4: *mut IP, hostname: *const i8, timeout: u32, cancel: *const bool) -> bool {
    return false;
}

// TODO: Unfishished
pub extern "C" fn GetIPEx(mut ip: *const IP, hostname: *const i8, timeout: u32, cancel: *const bool) -> bool {
    if (ip.is_null() || hostname.is_null() || cancel.is_null()) {
        return false;
    }

    // use read_volatile to read bool

    let hostname = unsafe{ CStr::from_ptr(hostname) };
    
    let ip = unsafe { &mut ip };
    
    return false;
}