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

use std::{collections::VecDeque, sync::Mutex};

struct Queue<T> {
    _internal: Mutex<VecDeque<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            _internal: Mutex::new(VecDeque::new()),
        }
    }

    pub fn as_mut_ptr(self) -> *mut Queue<T> {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    pub fn next(&mut self) -> Option<T> {
        let guard = self._internal.lock();

        let mut queue = guard.unwrap();

        queue.pop_front()
    }

    pub fn push(&mut self, item: T) {
        let guard = self._internal.lock();

        let mut queue = guard.unwrap();

        queue.push_back(item);
    }
}

pub extern "C" fn GetNext(q: *mut Queue) -> *mut c_void {}
pub extern "C" fn GetNextWithLock(q: *mut Queue) -> *mut c_void {}
pub extern "C" fn InsertQueue(q: *mut Queue, p: *mut c_void) {}
pub extern "C" fn InsertQueueWithLock(q: *mut Queue, p: *mut c_void) {}
pub extern "C" fn InsertQueueInt(q: *mut Queue, value: u32) {}
pub extern "C" fn LockQueue(q: *mut Queue) {}
pub extern "C" fn UnlockQueue(q: *mut Queue) {}
pub extern "C" fn ReleaseQueue(q: *mut Queue) {}
pub extern "C" fn CleanupQueue(q: *mut Queue) {}
pub extern "C" fn NewQueue() -> *mut Queue {}
pub extern "C" fn NewQueueFast() -> *mut Queue {}
pub extern "C" fn GetQueueNum(q: *mut Queue) -> u32 {}
