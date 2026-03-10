use std::ffi::{c_uint, c_ulong, c_ushort};

// USHORT Swap16(USHORT value);
#[unsafe(no_mangle)]
pub extern "C" fn Swap16(value: c_ushort) -> c_ushort {
    value.swap_bytes()
}

// UINT Swap32(UINT value);
#[unsafe(no_mangle)]
pub extern "C" fn Swap32(value: c_uint) -> c_uint {
    value.swap_bytes()
}

// UINT64 Swap64(UINT64 value);
#[unsafe(no_mangle)]
pub extern "C" fn Swap64(value: c_ulong) {
    value.swap_bytes();
}

// USHORT Endian16(USHORT src);
#[unsafe(no_mangle)]
pub extern "C" fn Endian16(value: c_ushort) -> c_ushort {
    value.to_be()
}

// UINT Endian32(UINT src);
#[unsafe(no_mangle)]
pub extern "C" fn Endian32(value: c_uint) -> c_uint {
    value.to_be()
}

// UINT64 Endian64(UINT64 src);
#[unsafe(no_mangle)]
pub extern "C" fn Endian64(value: c_ulong) -> c_ulong {
    value.to_be()
}

// USHORT LittleEndian16(USHORT src);
// UINT LittleEndian32(UINT src);
// UINT64 LittleEndian64(UINT64 src);
// void EndianUnicode(wchar_t *str);
