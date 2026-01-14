// Stack implementation not exported from mayaqua?

// SK *NewSK();
// SK *NewSKEx(bool no_compact);
// void ReleaseSK(SK *s);
// void CleanupSK(SK *s);
// void LockSK(SK *s);
// void UnlockSK(SK *s);
// void Push(SK *s, void *p);
// void *Pop(SK *s);

use std::{collections::VecDeque, sync::Mutex};

struct SK<T>{
    _internal: Mutex<VecDeque<T>>
}

impl<T> SK<T> {
    pub fn new() -> SK<T> {
        SK { _internal: Mutex::new(VecDeque::new())}
    }
}

pub extern "C" fn NewSK() -> *mut SK {}
pub extern "C" fn NewSKEx(no_compact: bool) -> *mut SK {}
pub extern "C" fn ReleaseSK(s: *mut SK) {}
pub extern "C" fn CleanupSK(s: *mut SK) {}
pub extern "C" fn LockSK(s: *mut SK) {}
pub extern "C" fn UnlockSK(s: *mut SK) {}
pub extern "C" fn Push(s: *mut SK, p: *mut c_void) {}
pub extern "C" fn Pop(s: *mut SK) -> *mut c_void {}
