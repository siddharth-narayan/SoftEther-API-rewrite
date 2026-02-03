use std::{
    ffi::c_char,
    thread::{sleep, yield_now},
    time::{self, Duration},
};

// void SleepThread(UINTtime)
pub extern "C" fn SleepThread(time: u32) {
    sleep(Duration::from_millis(time.into()));
}

// THREAD *NewThreadNamed(THREAD_PROC*thread_proc,void*param,char*name)
// void ReleaseThread(THREAD*t)
// void NoticeThreadInit(THREAD*t)
// void WaitThreadInit(THREAD*t)
// bool WaitThread(THREAD*t,UINTtimeout)
// void UINT64ToSystem(SYSTEMTIME*st,UINT64sec64)
// UINT64 SystemToUINT64(SYSTEMTIME*st)
// UINT64 LocalTime64()
// UINT64 SystemTime64()
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
// void GetDateTimeStrRFC3339(char*str,UINTsize,SYSTEMTIME*st,inttimezone_min)

pub extern "C" fn GetDateTimeStrRFC3339(str: *const u8, size: u32) {}
// bool Run(char*filename,char*arg,boolhide,boolwait)
// void GetSpanStrMilli(char*str,UINTsize,UINT64sec64)
// void GetMemInfo(MEMINFO*info)
// void YieldCpu()
pub extern "C" fn YieldCpu() {
    yield_now();
}

// LIST *NewThreadList()
// void AddThreadToThreadList(LIST*o,THREAD*t)
// void MaintainThreadList(LIST*o)
// void FreeThreadList(LIST*o)
// void StopThreadList(LIST*o)
// UINT GetNumberOfCpu()
