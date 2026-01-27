use std::collections::VecDeque;
use std::os::raw::c_void;
use std::ptr::null_mut;

use crate::mem::structs::buf::Buffer;
use crate::object::{RefCounter, Lock};
use crate::util::RawPtr;

pub struct Fifo<T> {
    ref_count: *mut RefCounter,
    lock: *mut Lock,
    p: *mut c_void,
    pos: u32,
    size: u32,
    memsize: u32,
    total_read_size: u64,
    total_write_size: u64,
    fixed: bool,

    // Rust internal
    _internal: VecDeque<T>,
}

impl<T: Copy> Fifo<T> {
    pub fn new() -> Fifo<T> {
        Fifo {
            ref_count: null_mut(),
            lock: null_mut(),
            p: null_mut(),
            pos: 0,
            size: 0,
            memsize: 0,
            total_read_size: 0,
            total_write_size: 0,
            fixed: false,

            _internal: VecDeque::new(),
        }
    }

    pub fn as_mut_ptr(self) -> *mut Fifo<T> {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Fifo<T>) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn size(&self) -> usize {
        self._internal.len()
    }

    pub fn read(&mut self, size: usize) -> Option<Vec<u8>> {
        return None;
    }

    pub fn read_all(&mut self) -> Buffer {
        let out = Buffer::new();

        out
    }

    pub fn write(&mut self, buf: &[T]) {
        let mut iter = buf.iter();

        while let Some(item) = iter.next() {
            self._internal.push_back(item.clone());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ReadFifo(ptr: *mut Fifo<RawPtr>, p: RawPtr, size: usize) {
    let fifo = unsafe { &mut *ptr };
}

#[unsafe(no_mangle)]
pub extern "C" fn ReadFifoAll(ptr: *mut Fifo<RawPtr>) -> *mut Buffer {
    let fifo = unsafe { &mut *ptr };

    fifo.read_all().as_mut_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn ShrinkFifoMemory(_: RawPtr) {
    // Leaving unimplemented because I don't care about shrinking memory
}

#[unsafe(no_mangle)]
pub extern "C" fn FifoPtr(ptr: *mut Fifo<RawPtr>) {
    let fifo = unsafe { &mut *ptr };
    // Send over underlying buf
}

#[unsafe(no_mangle)]
pub extern "C" fn WriteFifo(ptr: *mut Fifo<RawPtr>, content: RawPtr, size: usize) {
    let fifo = unsafe { &mut *ptr };
    // TODO: *copy* data from content pointer to Fifo
    // fifo.write()
}

#[unsafe(no_mangle)]
pub extern "C" fn FifoSize(ptr: *mut Fifo<RawPtr>) -> usize {
    let fifo = unsafe { &mut *ptr };

    fifo.size()
}

#[unsafe(no_mangle)]
pub extern "C" fn ReleaseFifo(ptr: *mut Fifo<RawPtr>) {
    let fifo = unsafe { &mut *ptr };
    Fifo::free_mut_ptr(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn NewFifo() -> *mut Fifo<RawPtr> {
    Fifo::<RawPtr>::new().as_mut_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn NewFifoFast() -> *mut Fifo<RawPtr> {
    NewFifo()
}

#[unsafe(no_mangle)]
pub extern "C" fn SetFifoCurrentReallocMemSize(size: usize) {
    // Can ignore?
}

// UINT ReadFifo(FIFO *f, void *p, UINT size);
// BUF *ReadFifoAll(FIFO *f);
// void ShrinkFifoMemory(FIFO *f);
// UCHAR *GetFifoPointer(FIFO *f); // Not exported?
// UCHAR *FifoPtr(FIFO *f);
// void WriteFifo(FIFO *f, void *p, UINT size);
// UINT FifoSize(FIFO *f);
// void ReleaseFifo(FIFO *f);
// void CleanupFifo(FIFO *f); // Not exported?
// FIFO *NewFifo();
// FIFO *NewFifoFast();
// FIFO *NewFifoEx(bool fast); // Not exported?
// FIFO *NewFifoEx2(bool fast, bool fixed); // Not exported?
// void InitFifo(); // Not exported?
// void SetFifoCurrentReallocMemSize(UINT size);
