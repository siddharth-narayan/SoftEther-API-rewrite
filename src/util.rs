use std::ffi::c_void;

pub type RawPtr = *mut c_void;

// UINT Base64Decode(void *dst, const void *src, const UINT size);
// UINT Base64Encode(void *dst, const void *src, const UINT size);