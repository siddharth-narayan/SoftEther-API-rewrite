// void *GetNext(QUEUE *q);
// void *GetNextWithLock(QUEUE *q);
// void InsertQueue(QUEUE *q, void *p);
// void InsertQueueWithLock(QUEUE *q, void *p);
// void InsertQueueInt(QUEUE *q, UINT value);
// void LockQueue(QUEUE *q);
// void UnlockQueue(QUEUE *q);
// void ReleaseQueue(QUEUE *q);
// void CleanupQueue(QUEUE *q);
// QUEUE *NewQueue();
// QUEUE *NewQueueFast();
// UINT GetQueueNum(QUEUE *q);

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