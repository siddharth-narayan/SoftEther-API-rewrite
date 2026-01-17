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

use crate::{mem::mem::{Clone, Copy}, util::RawPtr};

struct Queue<T> {
    lock: Mutex<()>,
    _internal: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            lock: Mutex::new(()),
            _internal: VecDeque::new(),
        }
    }

    pub fn as_mut_ptr(self) -> *mut Queue<T> {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    pub fn free_mut_ptr(ptr: *mut Queue<T>) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn len(&self) -> usize {
        self._internal.len()
    }

    pub fn next(&mut self) -> Option<T> {
        let guard = self.lock.lock();

        let mut queue = guard.unwrap();

        self._internal.pop_front()
    }

    pub fn push(&mut self, item: T) {
        let guard = self.lock.lock();

        self._internal.push_back(item);
    }
}

pub extern "C" fn GetNext(ptr: *mut Queue<RawPtr>) -> RawPtr {
    let queue = unsafe {&mut *ptr};

    if let Some(next) = queue.next() {
        next
    } else {
        null_mut()
    }
}

pub extern "C" fn GetNextWithLock(ptr: *mut Queue<RawPtr>) -> RawPtr {
    GetNext(ptr)
}

pub extern "C" fn InsertQueue(ptr: *mut Queue<RawPtr>, p: RawPtr) {
    let queue = unsafe {&mut *ptr};

    queue.push(p);
}

pub extern "C" fn InsertQueueWithLock(ptr: *mut Queue<RawPtr>, p: RawPtr) {
    InsertQueue(ptr, p);
}


pub extern "C" fn InsertQueueInt(ptr: *mut Queue<RawPtr>, value: u32) {
    let value_ptr: *const u32 = &value;
    let value_ptr = value_ptr as *const u8;
    
    let new_value = Clone(value_ptr, 4);
    let new_value = new_value as RawPtr;


    InsertQueue(ptr, new_value);
}

pub extern "C" fn LockQueue(ptr: *mut Queue<RawPtr>) {}
pub extern "C" fn UnlockQueue(ptr: *mut Queue<RawPtr>) {}

pub extern "C" fn ReleaseQueue(ptr: *mut Queue<RawPtr>) {
    Queue::free_mut_ptr(ptr);
}

pub extern "C" fn CleanupQueue(ptr: *mut Queue<RawPtr>) {
    Queue::free_mut_ptr(ptr); // Check difference with ReleaseQueue
}

pub extern "C" fn NewQueue() -> *mut Queue<RawPtr> {
    Queue::new().as_mut_ptr()
}

pub extern "C" fn NewQueueFast() -> *mut Queue<RawPtr> {
    Queue::new().as_mut_ptr()
}

pub extern "C" fn GetQueueNum(ptr: *mut Queue<RawPtr>) -> usize {
    let queue = unsafe {&mut *ptr};

    queue.len()
}
