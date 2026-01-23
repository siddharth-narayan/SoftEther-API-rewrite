use std::{ffi::c_char, thread::{sleep, yield_now}, time::{self, Duration}};

// SleepThread(UINTtime)
pub extern "C" fn SleepThread(time: u32) {
    sleep(Duration::from_millis(time.into()));
}
// NewThreadNamed(THREAD_PROC*thread_proc,void*param,char*name)
// ReleaseThread(THREAD*t)
// NoticeThreadInit(THREAD*t)
// WaitThreadInit(THREAD*t)
// WaitThread(THREAD*t,UINTtimeout)
// UINT64ToSystem(SYSTEMTIME*st,UINT64sec64)
// SystemToUINT64(SYSTEMTIME*st)
// LocalTime64()
// SystemTime64()
// LocalTime(SYSTEMTIME*st)
// SystemTime(SYSTEMTIME*st)
// GetDateTimeStrMilli(char*str,UINTsize,SYSTEMTIME*st)
// SystemToLocal64(UINT64t)
// LocalToSystem64(UINT64t)
// GetDateTimeStr64Uni(wchar_t*str,UINTsize,UINT64sec64)
// GetDateTimeStrMilli64(char*str,UINTsize,UINT64sec64)
// GetDateStr64(char*str,UINTsize,UINT64sec64)
// GetDateTimeStrEx64(wchar_t*str,UINTsize,UINT64sec64,LOCALE*locale)
// GetDateStrEx64(wchar_t*str,UINTsize,UINT64sec64,LOCALE*locale)
// GetTimeStrMilli64(char*str,UINTsize,UINT64sec64)
fn a(){
    time::SystemTime::now();
}
// GetDateTimeStrRFC3339(char*str,UINTsize,SYSTEMTIME*st,inttimezone_min)
pub extern "C" fn GetDateTimeStrRFC3339(str: *const u8, size: u32, ) {

}
// Run(char*filename,char*arg,boolhide,boolwait)
// GetSpanStrMilli(char*str,UINTsize,UINT64sec64)
// GetMemInfo(MEMINFO*info)
// YieldCpu()
pub extern "C" fn YieldCpu() {
    yield_now();
}
// NewThreadList()
// AddThreadToThreadList(LIST*o,THREAD*t)
// MaintainThreadList(LIST*o)
// FreeThreadList(LIST*o)
// StopThreadList(LIST*o)
// GetNumberOfCpu()
