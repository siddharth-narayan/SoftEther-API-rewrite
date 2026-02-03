// UINT DnsThreadNum()
// UINT DnsThreadNumMax()
// void DnsThreadNumMaxSet(constUINTnum)
// bool DnsResolve(IP*ipv6,IP*ipv4,constchar*hostname,UINTtimeout,volatileconstbool*cancel_flag)
// bool GetIPEx(IP*ip,constchar*hostname,UINTtimeout,volatileconstbool*cancel_flag)

use std::{
    ffi::CStr,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs},
};

use crate::{
    network::{structs::sock, util::IP},
    nullcheck,
};

pub fn resolve_ipv4(hostname: &str) -> Option<Ipv4Addr> {
    match resolve_all_ipv4(hostname) {
        Some(s) => {
            if s.len() < 1 {
                None
            } else {
                Some(s[0])
            }
        }
        None => None,
    }
}

pub fn resolve_ipv6(hostname: &str) -> Option<Ipv6Addr> {
    match resolve_all_ipv6(hostname) {
        Some(s) => {
            if s.len() < 1 {
                None
            } else {
                Some(s[0])
            }
        }
        None => None,
    }
}

pub fn resolve(hostname: &str) -> Option<IpAddr> {
    match resolve_all(hostname) {
        Some(s) => {
            if s.len() < 1 {
                None
            } else {
                Some(s[0])
            }
        }
        None => None,
    }
}

// The `resolve_all` methods may return a Vec with no elements
pub fn resolve_all_ipv4(hostname: &str) -> Option<Vec<Ipv4Addr>> {
    match resolve_all(hostname) {
        Some(s) => Some(
            s.into_iter()
                .filter_map(|s| match s {
                    IpAddr::V4(v4) => Some(v4),
                    IpAddr::V6(_) => None,
                })
                .collect(),
        ),
        None => None,
    }
}

pub fn resolve_all_ipv6(hostname: &str) -> Option<Vec<Ipv6Addr>> {
    match resolve_all(hostname) {
        Some(s) => Some(
            s.into_iter()
                .filter_map(|s| match s {
                    IpAddr::V4(_) => None,
                    IpAddr::V6(v6) => Some(v6),
                })
                .collect(),
        ),
        None => None,
    }
}

pub fn resolve_all(hostname: &str) -> Option<Vec<IpAddr>> {
    match (hostname, 0).to_socket_addrs() {
        Ok(socket_iter) => Some(socket_iter.map(|s| s.ip()).collect()),
        Err(e) => None,
    }
}

pub extern "C" fn DnsThreadNum() -> u32 {
    return 1;
}

pub extern "C" fn DnsThreadNumMax() -> u32 {
    return 8;
}

pub extern "C" fn DnsThreadNumMaxSet(num: u32) {}

pub extern "C" fn DnsResolve(
    ipv6: *mut IP,
    ipv4: *mut IP,
    hostname: *const i8,
    timeout: u32,
    cancel: *const bool,
) -> bool {
    return false;
}

// TODO: Unfishished
pub extern "C" fn GetIPEx(
    mut ip: *const IP,
    hostname: *const i8,
    timeout: u32,
    cancel: *const bool,
) -> bool {
    nullcheck!(false, ip, hostname, cancel);

    // use read_volatile to read bool ?

    let hostname = unsafe { CStr::from_ptr(hostname) };

    let ip = unsafe { &mut ip };

    return false;
}
