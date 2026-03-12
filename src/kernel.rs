use std::{
    ffi::{c_char, c_void}, process::{abort, exit}, thread::{sleep, yield_now}, time::{self, Duration}
};

use crate::{mem::structs::list::List, util::RawPtr};

// Mayaqua.h

// void InitProcessCallOnce()
#[unsafe(no_mangle)]
pub extern "C" fn InitProcessCallOnce() {

}

// bool Is64()
#[unsafe(no_mangle)]
pub extern "C" fn Is64() -> bool {
    std::mem::size_of::<usize>() == 8
}

// void Alert(char*msg,char*caption)
#[unsafe(no_mangle)]
pub extern "C" fn Alert(msg: *mut c_char, caption: *mut c_char) {

}

// OS_INFO *GetOsInfo()

// Kernel.h

// THREAD *NewThreadNamed(THREAD_PROC*thread_proc,void*param,char*name)
// void ReleaseThread(THREAD*t)
// void NoticeThreadInit(THREAD*t)
// void WaitThreadInit(THREAD*t)
// bool WaitThread(THREAD*t,UINTtimeout)
// void UINT64ToSystem(SYSTEMTIME*st,UINT64sec64)
// UINT64 SystemToUINT64(SYSTEMTIME*st)
// void LocalTime(SYSTEMTIME*st)
// void SystemTime(SYSTEMTIME*st)
// void GetDateTimeStrMilli(char*str,UINTsize,SYSTEMTIME*st)
// UINT64 SystemToLocal64(UINT64t)
// UINT64 LocalToSystem64(UINT64t)
// void GetDateTimeStr64Uni(wchar_t*str,UINTsize,UINT64sec64)
// void GetDateTimeStrMilli64(char*str,UINTsize,UINT64sec64)
// void GetDateStr64(char*str,UINTsize,UINT64sec64)
// void GetDateTimeStrEx64(wchar_t*str,UINTsize,UINT64sec64,LOCALE*locale)
// void GetDateStrEx64(wchar_t*str,UINTsize,UINT64sec64,LOCALE*locale)
// void GetTimeStrMilli64(char*str,UINTsize,UINT64sec64)
// bool Run(char*filename,char*arg,boolhide,boolwait)
// void AbortExit()
#[unsafe(no_mangle)]
pub extern "C" fn AbortExit() {
    abort()
}

// void SleepThread(UINTtime)
#[unsafe(no_mangle)]
pub extern "C" fn SleepThread(time: u32) {
    sleep(Duration::from_millis(time.into()));
}

// UINT64 LocalTime64()
#[unsafe(no_mangle)]
pub extern "C" fn LocalTime64() -> u64 {
    0
}

// UINT64 SystemTime64()
#[unsafe(no_mangle)]
pub extern "C" fn SystemTime64() -> u64 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn GetDateTimeStrRFC3339(str: *const u8, size: u32) {}

#[unsafe(no_mangle)]
pub extern "C" fn GetSpanStrMilli(str: *const char, size: u32, sec: u64) {
    todo!()
}

// void GetMemInfo(MEMINFO*info)
#[unsafe(no_mangle)]
pub extern "C" fn GetMemInfo(info: RawPtr) {
    todo!()
}

// void YieldCpu()
#[unsafe(no_mangle)]
pub extern "C" fn YieldCpu() {
    yield_now();
}

// LIST *NewThreadList()
#[unsafe(no_mangle)]
pub extern "C" fn NewThreadList(o: *mut List<RawPtr>) {
    todo!()
}

// void AddThreadToThreadList(LIST*o,THREAD*t)
#[unsafe(no_mangle)]
pub extern "C" fn AddThreadToThreadList(o: *mut List<RawPtr>,  t: *mut c_void) {
    todo!()
}


// void MaintainThreadList(LIST*o)
#[unsafe(no_mangle)]
pub extern "C" fn MaintainThreadList(o: *mut List<RawPtr>) {
    todo!()
}

// void FreeThreadList(LIST*o)
#[unsafe(no_mangle)]
pub extern "C" fn FreeThreadList(o: *mut List<RawPtr>) {
    todo!()
}

// void StopThreadList(LIST*o)
#[unsafe(no_mangle)]
pub extern "C" fn StopThreadList(o: *mut List<RawPtr>) {
    todo!()
}

// UINT GetNumberOfCpu()
#[unsafe(no_mangle)]
pub extern "C" fn GetNumberOfCpu() -> u32 {
    2
}
