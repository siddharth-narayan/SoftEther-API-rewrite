// void *GetUnixio4Stdout()
// void UnixSetHighOomScore()
// TOKEN_LIST *UnixExec(char*cmd)
// void UnixDisableInterfaceOffload(char*name)
// void UnixSetEnableKernelEspProcessing(boolb)
// void UnixDisableCoreDump()
// void UnixCloseIO()
// bool UnixIsInVm()

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

#[unsafe(no_mangle)]
pub extern "C" fn GetUnixio4Stdout() -> RawPtr {
    static OUT: UNIXIO = UNIXIO {
        fd: 1,
        write_mode: true,
    };

    &OUT as *const UNIXIO as RawPtr
}

#[unsafe(no_mangle)]
pub extern "C" fn UnixSetHighOomScore() {
    let path = format!("/proc/{}/oom_score_adj", process::id());

    if let Ok(mut file) = File::create(path) {
        file.write("800".as_bytes());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn UnixExec(command: *const c_char) {}

#[unsafe(no_mangle)]
pub extern "C" fn UnixDisableInterfaceOffload(name: *const c_char) {}

#[unsafe(no_mangle)]
pub extern "C" fn UnixSetEnableKernelEspProcessing(b: bool) {}

#[unsafe(no_mangle)]
pub extern "C" fn UnixDisableCoreDump() {}

#[unsafe(no_mangle)]
pub extern "C" fn UnixCloseIO() {}

#[unsafe(no_mangle)]
pub extern "C" fn UnixIsInVM() -> bool {
    return false;
}
