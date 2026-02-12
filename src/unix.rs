use std::{
    ffi::{CStr, c_char},
    fs::File,
    io::Write,
    process,
};

use crate::util::RawPtr;

#[repr(C)]
struct UNIXIO {
    fd: i32,
    write_mode: bool,
}

// void *GetUnixio4Stdout()
#[unsafe(no_mangle)]
pub extern "C" fn GetUnixio4Stdout() -> RawPtr {
    static OUT: UNIXIO = UNIXIO {
        fd: 1,
        write_mode: true,
    };

    &OUT as *const UNIXIO as RawPtr
}

// void UnixSetHighOomScore()
#[unsafe(no_mangle)]
pub extern "C" fn UnixSetHighOomScore() {
    let path = format!("/proc/{}/oom_score_adj", process::id());

    if let Ok(mut file) = File::create(path) {
        file.write("800".as_bytes());
    }
}

// TOKEN_LIST *UnixExec(char*cmd)
#[unsafe(no_mangle)]
pub extern "C" fn UnixExec(command: *const c_char) {}

// void UnixDisableInterfaceOffload(char*name)
#[unsafe(no_mangle)]
pub extern "C" fn UnixDisableInterfaceOffload(name: *const c_char) {}

// void UnixSetEnableKernelEspProcessing(boolb)
#[unsafe(no_mangle)]
pub extern "C" fn UnixSetEnableKernelEspProcessing(b: bool) {}

// void UnixDisableCoreDump()
#[unsafe(no_mangle)]
pub extern "C" fn UnixDisableCoreDump() {}

// void UnixCloseIO()
#[unsafe(no_mangle)]
pub extern "C" fn UnixCloseIO() {}

// bool UnixIsInVm()
#[unsafe(no_mangle)]
pub extern "C" fn UnixIsInVM() -> bool {
    return false;
}
