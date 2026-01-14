// SK *NewSk();
// SK *NewSkEx(bool no_compact);
// void ReleaseSk(SK *s);
// void CleanupSk(SK *s);
// void LockSk(SK *s);
// void UnlockSk(SK *s);
// void Push(SK *s, void *p);
// void *Pop(SK *s);

pub extern "C" fn NewSk() -> *mut Sk {}
pub extern "C" fn NewSkEx(no_compact: bool) -> *mut Sk {}
pub extern "C" fn ReleaseSk(s: *mut Sk) {}
pub extern "C" fn CleanupSk(s: *mut Sk) {}
pub extern "C" fn LockSk(s: *mut Sk) {}
pub extern "C" fn UnlockSk(s: *mut Sk) {}
pub extern "C" fn Push(s: *mut Sk, p: *mut c_void) {}
pub extern "C" fn Pop(s: *mut Sk) -> *mut c_void {}
