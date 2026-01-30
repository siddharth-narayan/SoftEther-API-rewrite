// void *GetNext(QUEUE *q);
// void *GetNextWithLock(QUEUE *q); // Not exported?
// void InsertQueue(QUEUE *q, void *p); a
// void InsertQueueWithLock(QUEUE *q, void *p); // Not exported?
// void InsertQueueInt(QUEUE *q, UINT value); // Not exported?
// void LockQueue(QUEUE *q); a
// void UnlockQueue(QUEUE *q);a
// void ReleaseQueue(QUEUE *q); a
// void CleanupQueue(QUEUE *q); // Not exported?
// QUEUE *NewQueue(); a
// QUEUE *NewQueueFast(); a
// UINT GetQueueNum(QUEUE *q); a

use std::{collections::VecDeque, ffi::c_void, ptr::null_mut, sync::Mutex};

use crate::{
    mem::{
        mem::{Clone, Copy},
        structs::fifo::Fifo,
    },
    object::{Lock, RefCounter},
    util::RawPtr,
};

pub struct Queue<T> {
    ref_count: *mut RefCounter,
    size: u32,
    fifo: *mut Fifo<T>,
    lock: *mut Lock,

    _lock: Lock,
    _internal: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self<T> {
        Self {
            ref_count: null_mut(),
            size: 0,
            fifo: null_mut(),
            lock: null_mut(),

            _internal: VecDeque::new(),
            _lock: Lock::new()
        }
    }

    pub fn as_mut_ptr(self) -> *mut Self<T> {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Self<T>) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn len(&self) -> usize {
        self._internal.len()
    }

    pub fn next(&mut self) -> Option<T> {
        let guard = self._lock.lock();

        // let mut queue = guard.unwrap();

        self._internal.pop_front()
    }

    pub fn push(&mut self, item: T) {
        // let guard = self.lock.lock();

        self._internal.push_back(item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn GetNext(ptr: *mut Queue<RawPtr>) -> RawPtr {
    let queue = unsafe { &mut *ptr };

    if let Some(next) = queue.next() {
        next
    } else {
        null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn GetNextWithLock(ptr: *mut Queue<RawPtr>) -> RawPtr {
    GetNext(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn InsertQueue(ptr: *mut Queue<RawPtr>, p: RawPtr) {
    let queue = unsafe { &mut *ptr };

    queue.push(p);
}

#[unsafe(no_mangle)]
pub extern "C" fn InsertQueueWithLock(ptr: *mut Queue<RawPtr>, p: RawPtr) {
    InsertQueue(ptr, p);
}

#[unsafe(no_mangle)]
pub extern "C" fn InsertQueueInt(ptr: *mut Queue<RawPtr>, value: u32) {
    let value_ptr: *const u32 = &value;
    let value_ptr = value_ptr as *const u8;

    let new_value = Clone(value_ptr, 4);
    let new_value = new_value as RawPtr;

    InsertQueue(ptr, new_value);
}

#[unsafe(no_mangle)]
pub extern "C" fn LockQueue(ptr: *mut Queue<RawPtr>) {}

#[unsafe(no_mangle)]
pub extern "C" fn UnlockQueue(ptr: *mut Queue<RawPtr>) {}

#[unsafe(no_mangle)]
pub extern "C" fn ReleaseQueue(ptr: *mut Queue<RawPtr>) {
    Queue::free_mut_ptr(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn CleanupQueue(ptr: *mut Queue<RawPtr>) {
    Queue::free_mut_ptr(ptr); // Check difference with ReleaseQueue
}

#[unsafe(no_mangle)]
pub extern "C" fn NewQueue() -> *mut Queue<RawPtr> {
    Queue::new().as_mut_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn NewQueueFast() -> *mut Queue<RawPtr> {
    Queue::new().as_mut_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn GetQueueNum(ptr: *mut Queue<RawPtr>) -> usize {
    let queue = unsafe { &mut *ptr };

    queue.len()
}
