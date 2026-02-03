// Simple memory API, like allocating, freeing, copy, etc

// void CheckMemTag1(MEMTAG1 *tag);
// void CheckMemTag2(MEMTAG2 *tag);

// int CmpCaseIgnore(void *p1, void *p2, UINT size);
// void *AddHead(void *src, UINT src_size, void *head, UINT head_size);

use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ffi::{c_uint, c_void};
use std::mem::zeroed;
use std::ptr::{null_mut, slice_from_raw_parts};
use std::{mem, ptr, slice, usize};

use crate::nullcheck;

// TODO: Some of the functions modify stats in the Kernel -- Like a count of allocations and frees, to detect memory leaks

const HEADER_SIZE: usize = mem::size_of::<c_void>();
const HEADER_MAGIC_ALLOCATED: u32 = 0x67676767; // This will not be funny a year from now 
const HEADER_MAGIC_UNALLOCATED: u32 = 0x76767676;

const LAYOUT_ALIGN: usize = align_of::<Header>();

#[repr(C)]
struct Header {
    magic_byte: u32, // So we don't manage pointers not created by us
    size: u32,
    zero_after_free: bool,
}

impl Header {
    pub fn get_layout(&mut self) -> Option<Layout> {
        match Layout::from_size_align(HEADER_SIZE + self.size as usize, LAYOUT_ALIGN) {
            Ok(l) => Some(l),
            Err(e) => None,
        }
    }

    
    pub fn get_header_ptr(user_ptr: *mut u8) -> *mut Header {
        unsafe { user_ptr.sub(HEADER_SIZE) as *mut Header }
    }

    pub fn get_user_ptr(header_ptr: *mut Header) -> *mut u8 {
        let header_ptr = header_ptr as *mut u8;
        unsafe { header_ptr.add(HEADER_SIZE) }
    }
}

// void *Malloc(UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Malloc(size: u32) -> *mut u8 {
    MallocEx(size, false)
}

// void *MallocEx(UINT size, bool zero_clear_when_free);
#[unsafe(no_mangle)]
pub extern "C" fn MallocEx(size: u32, zero_after_free: bool) -> *mut u8 {
    let layout = Layout::from_size_align(HEADER_SIZE + size as usize, LAYOUT_ALIGN).unwrap();

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return null_mut();
    }

    // Store size in header
    let header_ptr = ptr as *mut Header;
    let header = unsafe { &mut *header_ptr };

    header.magic_byte = HEADER_MAGIC_ALLOCATED;   
    header.size = size;
    header.zero_after_free = zero_after_free;

    // Return pointer after header
    unsafe { Header::get_user_ptr(header_ptr) }
}

// void *InternalMalloc(UINT size);
#[inline]
#[unsafe(no_mangle)]
pub extern "C" fn InternalMalloc(size: u32) -> *mut u8 {
    Malloc(size)
} 

// void *ZeroMalloc(UINT size);
pub extern "C" fn ZeroMalloc(size: u32) -> *mut u8 {
    ZeroMallocEx(size, false)
}
// void *ZeroMallocEx(UINT size, bool zero_clear_when_free);

#[unsafe(no_mangle)]
pub extern "C" fn ZeroMallocEx(size: u32, zero_after_free: bool) -> *mut u8 {
    let address = MallocEx(size, zero_after_free);

    if address.is_null() {
        return null_mut();
    }

    let slice = unsafe { slice::from_raw_parts_mut(address, size as usize) };
    slice.fill(0);


    address
}

// void *ReAlloc(void *addr, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn ReAlloc(user_ptr: *mut u8, new_size: u32) -> *mut u8 {
    nullcheck!(user_ptr, user_ptr);

    let header_ptr = Header::get_header_ptr(user_ptr);
    let header = unsafe { &mut *header_ptr };
    
    let layout = match header.get_layout() {
        Some(l) => l,
        None => {
            println!("Failed to build layout");
            return user_ptr;
        },
    };

    if header.magic_byte != HEADER_MAGIC_ALLOCATED {
        return user_ptr
    }

    let new_ptr = unsafe { realloc(header_ptr as *mut u8, layout, HEADER_SIZE + new_size as usize) };
    if new_ptr.is_null() {
        return user_ptr
    }

    let header_ptr = new_ptr as *mut Header;
    let header = unsafe { &mut *header_ptr };

    header.size = new_size;

    unsafe { Header::get_user_ptr(header_ptr) }
}

// void *InternalReAlloc(void *addr, UINT size);
#[inline]
#[unsafe(no_mangle)]
pub extern "C" fn InternalReAlloc(user_ptr: *mut u8, size: u32) -> *mut u8 {
    ReAlloc(user_ptr, size)
}

// void Free(void *addr);
#[unsafe(no_mangle)]
pub extern "C" fn Free(user_ptr: *mut u8) {
    nullcheck!((), user_ptr);

    let header_ptr = Header::get_header_ptr(user_ptr);
    let header = unsafe { &mut *header_ptr };

    if header.magic_byte != HEADER_MAGIC_ALLOCATED {
        return
    }

    let layout = match header.get_layout() {
        Some(l) => l,
        None => {
            return;
        }
    };

    if header.zero_after_free {
        Zero(user_ptr, header.size);
    }
    
    header.magic_byte = HEADER_MAGIC_UNALLOCATED;
    
    unsafe { dealloc(header_ptr as *mut u8, layout) };
}

// void InternalFree(void *addr);
#[inline]
#[unsafe(no_mangle)]
pub extern "C" fn InternalFree(user_ptr: *mut u8) {
    Free(user_ptr);
}

// void Move(void *dst, void *src, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Move(destination: *mut u8, source: *mut u8, size: u32) {
    nullcheck!((), destination, source);

    if size == 0 {
        return;
    }

    // ptr::copy is equivalent to memmove in C
    unsafe { ptr::copy(source, destination, size as usize) };
}

// void Copy(void *dst, void *src, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Copy(destination: *mut u8, source: *const u8, size: u32) {
    nullcheck!((), destination, source);

    if size == 0 {
        return;
    }

    // ptr::copy is equivalent to memcpy in C
    unsafe { ptr::copy_nonoverlapping(source, destination, size as usize) };
}

// void *Clone(void *addr, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Clone(source: *const u8, size: u32) -> *mut u8 {
    nullcheck!(null_mut(), source);

    let clone = Malloc(size);
    nullcheck!(null_mut(), clone);

    Copy(clone, source, size);

    return clone;
}

// void Zero(void *addr, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Zero(addr: *mut u8, size: u32) {
    nullcheck!((), addr);

    unsafe {
        ptr::write_bytes(addr, 0, size as usize);
    }
}

// UINT GetMemSize(void *addr);
#[unsafe(no_mangle)]
pub extern "C" fn GetMemSize(addr: *mut u8) -> u32 {
    nullcheck!(0, addr);

    let header = Header::get_header_ptr(addr);
    let header = unsafe { &mut *header };

    if header.magic_byte != HEADER_MAGIC_ALLOCATED {
        return 0;
    }

    header.size
}

// int Cmp(void *p1, void *p2, UINT size);
#[unsafe(no_mangle)]
pub extern "C" fn Cmp(left: *const u8, right: *const u8, size: u32) -> i32 {
    nullcheck!(0, left, right);
    let left = unsafe { slice::from_raw_parts(left, size as usize) };
    let right = unsafe { slice::from_raw_parts(right, size as usize) };

    match left.cmp(right) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}