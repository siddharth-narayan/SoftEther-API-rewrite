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

use crate::mem::mem::{Clone, Copy};

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

pub extern "C" fn GetNext(ptr: *mut Queue<*mut c_void>) -> *mut c_void {
    let queue = unsafe {&mut *ptr};

    if let Some(next) = queue.next() {
        next
    } else {
        null_mut()
    }
}

pub extern "C" fn GetNextWithLock(ptr: *mut Queue<*mut c_void>) -> *mut c_void {
    GetNext(ptr)
}

pub extern "C" fn InsertQueue(ptr: *mut Queue<*mut c_void>, p: *mut c_void) {
    let queue = unsafe {&mut *ptr};

    queue.push(p);
}

pub extern "C" fn InsertQueueWithLock(ptr: *mut Queue<*mut c_void>, p: *mut c_void) {
    InsertQueue(ptr, p);
}


pub extern "C" fn InsertQueueInt(ptr: *mut Queue<*mut c_void>, value: u32) {
    let value_ptr: *const u32 = &value;
    let value_ptr = value_ptr as *const u8;
    
    let new_value = Clone(value_ptr, 4);
    let new_value = new_value as *mut c_void;


    InsertQueue(ptr, new_value);
}

pub extern "C" fn LockQueue(ptr: *mut Queue<*mut c_void>) {}
pub extern "C" fn UnlockQueue(ptr: *mut Queue<*mut c_void>) {}

pub extern "C" fn ReleaseQueue(ptr: *mut Queue<*mut c_void>) {
    Queue::free_mut_ptr(ptr);
}

pub extern "C" fn CleanupQueue(ptr: *mut Queue<*mut c_void>) {
    Queue::free_mut_ptr(ptr); // Check difference with ReleaseQueue
}

pub extern "C" fn NewQueue() -> *mut Queue<*mut c_void> {
    Queue::new().as_mut_ptr()
}

pub extern "C" fn NewQueueFast() -> *mut Queue<*mut c_void> {
    Queue::new().as_mut_ptr()
}

pub extern "C" fn GetQueueNum(ptr: *mut Queue<*mut c_void>) -> usize {
    let queue = unsafe {&mut *q};

    queue.len()
}
