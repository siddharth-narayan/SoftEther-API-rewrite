use std::{cmp::min, ffi::{c_char, c_void}};

pub type RawPtr = *mut c_void;
pub type RawCStr = *mut c_char;

pub trait CCompat {
    fn c_compat(&mut self);
}

/// Safe `copy_from_slice` wrapper that should never panic.
/// Copies the maximum amount of elements it can safely from one slice to another
pub fn copy<T: Copy>(dst: &mut [T], src: &[T]) -> usize {
    let copy_len = min(src.len(), dst.len());

    let src = &src[0..copy_len];
    let dst = &mut dst[0..copy_len];
    dst.copy_from_slice(src);

    copy_len
}

// UINT Base64Decode(void *dst, const void *src, const UINT size);
// UINT Base64Encode(void *dst, const void *src, const UINT size);
