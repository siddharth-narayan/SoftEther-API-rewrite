// Simple memory API, like allocating, freeing, copy, etc

// void *Malloc(UINT size);
// void *MallocEx(UINT size, bool zero_clear_when_free);
// void *ZeroMalloc(UINT size);
// void *ZeroMallocEx(UINT size, bool zero_clear_when_free);
// void *ReAlloc(void *addr, UINT size);
// void Free(void *addr);
// void FreeSafe(void **addr);
// void CheckMemTag1(MEMTAG1 *tag);
// void CheckMemTag2(MEMTAG2 *tag);
// UINT GetMemSize(void *addr);


// void *InternalMalloc(UINT size);
// void *InternalReAlloc(void *addr, UINT size);
// void InternalFree(void *addr);

// void Copy(void *dst, void *src, UINT size);
// void Move(void *dst, void *src, UINT size);
// int Cmp(void *p1, void *p2, UINT size);
// int CmpCaseIgnore(void *p1, void *p2, UINT size);
// void ZeroMem(void *addr, UINT size);
// void Zero(void *addr, UINT size);
// void *Clone(void *addr, UINT size);
// void *AddHead(void *src, UINT src_size, void *head, UINT head_size);


use std::alloc::{Layout, alloc, dealloc};
use std::ffi::{c_uint, c_void};
use std::mem::zeroed;
use std::{mem, ptr};

#[repr(C)]
struct Header {
    size: usize,
}

extern "C" fn Malloc(size: usize) -> *mut u8 {
    MallocEx(size, false)
}

extern "C" fn MallocEx(size: usize, zero_after_free: bool) -> *mut u8 {
    let header_size = mem::size_of::<Header>();
    let layout = Layout::from_size_align(header_size + size, mem::align_of::<usize>()).unwrap();

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    // Store size in header
    let header_ptr = ptr as *mut Header;
    unsafe {
        (*header_ptr).size = size;
    }

    // Return pointer after header
    unsafe { ptr.add(header_size) }
}

// TODO: The following functions modify stats in the Kernel -- Like a count of allocations and frees, to detect memory leaks

extern "C" fn Free(user_ptr: *mut u8) {
    let header_size = mem::size_of::<Header>();
    let header_ptr = unsafe { user_ptr.sub(header_size) as *mut Header };
    let size = unsafe { (*header_ptr).size };

    let layout = Layout::from_size_align(header_size + size, mem::align_of::<usize>()).unwrap();
    unsafe { dealloc(header_ptr as *mut u8, layout) };
}

extern "C" fn Move(destination: *mut u8, source: *mut u8, size: usize) {
    if destination.is_null() || source.is_null() || size == 0 {
        return;
    }

    // ptr::copy is equivalent to memmove in C
    unsafe { ptr::copy(source, destination, size) };
}

extern "C" fn Copy(destination: *mut u8, source: *mut u8, size: usize) {
    if destination.is_null() || source.is_null() || size == 0 {
        return;
    }

    for index in 0..size {
        unsafe {
            *destination.add(index.into()) = *source.add(index.into());
        }
    }
}

extern "C" fn Clone(source: *mut u8, size: usize) -> *mut u8 {
    let clone = Malloc(size);

    Copy(clone, source, size);

    return clone;
}

extern "C" fn Zero(addr: *mut u8, size: usize) {
    if addr.is_null() {
        return;
    }

    unsafe {
        ptr::write_bytes(addr, 0, size);
    }
}
