use std::{cmp::min, ffi::{c_char, c_void}};

pub type RawPtr = *mut c_void;
pub type RawCStr = *mut c_char;

pub trait CCompat {
    fn c_compat(&mut self);
}

pub fn copy_slice_to_slice<T: Copy>(dst: &mut [T], src: &[T], size: usize) {
    let copy_len = min(min(dst.len(), src.len()), size);
    dst.copy_from_slice(src);
}

// UINT Base64Decode(void *dst, const void *src, const UINT size);
// UINT Base64Encode(void *dst, const void *src, const UINT size);
