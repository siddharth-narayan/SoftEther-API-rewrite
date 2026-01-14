// BUF *NewBuf();
// BUF *NewBufFromMemory(void *buf, UINT size);
// void ClearBuf(BUF *b);

// bool BufSkipUtf8Bom(BUF *b);

// void AdjustBufSize(BUF *b, UINT new_size);
// void SeekBuf(BUF *b, UINT offset, int mode);
// void SeekBufToEnd(BUF *b);
// void SeekBufToBegin(BUF *b);
// void FreeBuf(BUF *b);
// bool BufToFile(IO *o, BUF *b);
// BUF *FileToBuf(IO *o);

// void AddBufStr(BUF *b, char *str);
// bool DumpBuf(BUF *b, char *filename);
// bool DumpBufW(BUF *b, wchar_t *filename);
// bool DumpBufWIfNecessary(BUF *b, wchar_t *filename);
// bool DumpDataW(void *data, UINT size, wchar_t *filename);

// BUF *CloneBuf(BUF *b);
// BUF *MemToBuf(void *data, UINT size);
// BUF *RandBuf(UINT size);

// bool CompareBuf(BUF *b1, BUF *b2);

// SHARED_BUFFER *NewSharedBuffer(void *data, UINT size);
// void ReleaseSharedBuffer(SHARED_BUFFER *b);
// void CleanupSharedBuffer(SHARED_BUFFER *b);

// void AppendBufUtf8(BUF *b, wchar_t *str);
// void AppendBufStr(BUF *b, char *str);

use std::{ffi::CStr, fs::File, io::{Read, Write}, path::Path, ptr::{null, null_mut}};

struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buf: Vec::new(),
            pos: 0,
        }
    }

    pub fn as_mut_ptr(self) -> *mut Buffer {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    pub fn seek(&mut self, position: usize) {
        self.pos = position;
    }

    pub fn seek_offset(&mut self, offset: isize) {
        self.pos = self.pos.saturating_add_signed(offset)
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }
}

impl Buffer {
    pub fn read(&mut self, mut size: usize) -> &mut [u8] {
        if (self.buf.len() - 1) - self.pos < size {
            return &mut self.buf[self.pos..];
        }

        &mut self.buf[self.pos..self.pos + size]
    }

    // Reads a full buffer into another buff
    pub fn read_buf(&mut self, buffer: &mut Buffer, size: usize) {
        if let data = self.read(size) {
            buffer.write(data);
        }
    }

    pub fn read_buf_offset(&mut self, buffer: &Buffer) {
        // if let Some(data) = self.read(size) {
        //     buffer.write(data);
        // }
    }

    pub fn read_u8(&mut self) -> u8 {
        let slice = self.read(1).try_into().unwrap_or_default();

        u8::from_be_bytes(slice)
    }

    pub fn read_u16(&mut self) -> u16 {
        let slice = self.read(2).try_into().unwrap_or_default();

        u16::from_be_bytes(slice)
    }

    pub fn read_u32(&mut self) -> u32 {
        let slice = self.read(4).try_into().unwrap_or_default();

        u32::from_be_bytes(slice)
    }

    pub fn read_u64(&mut self) -> u64 {
        let slice = self.read(8).try_into().unwrap_or_default();

        u64::from_be_bytes(slice)
    }

    pub fn read_str(&mut self, str: &mut str) -> Option<&str> {
        let string_length = self.read_u32();

        let string_bytes = self.read(string_length.try_into().unwrap());
        if let Ok(string) = str::from_utf8(string_bytes) {
            return Some(string);
        }

        return None;
    }
}

// Write methods
// void WriteBuf(BUF *b, void *buf, UINT size);
// void WriteBufBuf(BUF *b, BUF *bb);
// void WriteBufBufWithOffset(BUF *b, BUF *bb);
// bool WriteBufInt(BUF *b, UINT value);
// bool WriteBufInt64(BUF *b, UINT64 value);
// bool WriteBufChar(BUF *b, UCHAR uc);
// bool WriteBufShort(BUF *b, USHORT value);
// bool WriteBufStr(BUF *b, char *str);
// void WriteBufLine(BUF *b, char *str);
impl Buffer {
    pub fn write(&mut self, data: &[u8]) {
        self.buf.write_all(data);
    }

    pub fn write_buf(&mut self, buffer: &Buffer) {
        self.write(&buffer.clone().buf.as_slice());
    }

    pub fn write_buf_offset(&mut self, buffer: &Buffer) {
        self.write(&buffer.clone().buf[buffer.pos..]);
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.write([byte; 1].as_slice());
    }

    pub fn write_u16(&mut self, int: u16) {
        self.write(int.to_be_bytes().as_slice());
    }

    pub fn write_u32(&mut self, int: u32) {
        self.write(int.to_be_bytes().as_slice());
    }

    pub fn write_u64(&mut self, int: u64) {
        self.write(int.to_be_bytes().as_slice());
    }

    pub fn write_str(&mut self, str: String) {
        self.write(str.as_bytes());
    }

    pub fn write_line(&mut self, mut str: String) {
        str.insert_str(str.len(), "\r\n");

        self.write_u32(str.len() as u32);
        self.write(str.as_bytes());
    }
}

// ==================================================
// ==================== C API =======================
// ==================================================

// UINT ReadBuf(BUF *b, void *buf, UINT size);
// BUF *ReadBufFromBuf(BUF *b, UINT size);
// Read methods
// UINT ReadBufInt(BUF *b);
// USHORT ReadBufShort(BUF *b);
// UINT64 ReadBufInt64(BUF *b);
// UCHAR ReadBufChar(BUF *b);

// BUF *ReadDump(char *filename);
// BUF *ReadDumpWithMaxSize(char *filename, UINT max_size);
// BUF *ReadDumpW(wchar_t *filename);
// BUF *ReadDumpExW(wchar_t *filename, bool read_lock);
// bool ReadBufStr(BUF *b, char *str, UINT size);
// BUF *ReadRemainBuf(BUF *b);
// UINT ReadBufRemainSize(BUF *b);
use crate::mem::mem::{Copy, Zero};

pub extern "C" fn ReadBuf(buffer: *mut Buffer, out: *mut u8, size: usize) -> usize {
    let buffer = unsafe { &mut *buffer };

    // SoftEther implementation writes 0's to the output buffer
    // Can be optimized to only zero unwritten portion, after read
    Zero(out, size);

    let slice = buffer.read(size);

    Copy(out, slice.as_mut_ptr(), slice.len());

    slice.len()
}

pub extern "C" fn ReadBufFromBuf(source: *mut Buffer, size: usize) -> *mut Buffer {
    let source = unsafe { &mut *source };

    let mut new = Buffer::new();

    let data = source.read(size);

    if data.len() < size {
        return std::ptr::null_mut()
    }

    new.write(data);
    new.as_mut_ptr()
}

pub extern "C" fn ReadBufChar(buffer: *mut Buffer) -> u8 {
    let buffer = unsafe { &mut *buffer };

    buffer.read_u8()
}

pub extern "C" fn ReadBufShort(buffer: *mut Buffer) -> u16 {
    let buffer = unsafe { &mut *buffer };

    buffer.read_u16()
}

pub extern "C" fn ReadBufInt(buffer: *mut Buffer) -> u32 {
    let buffer = unsafe { &mut *buffer };

    buffer.read_u32()
}

pub extern "C" fn ReadBufInt64(buffer: *mut Buffer) -> u64 {
    let buffer = unsafe { &mut *buffer };

    buffer.read_u64()
}

pub extern "C" fn ReadDump(filename: *const i8) -> *mut Buffer {
    let filename = unsafe { CStr::from_ptr(filename) };

    let filename = match filename.to_str() {
        Ok(s) => s,
        Err(_) => return null_mut(), // invalid UTF-8
    };

    if let Ok(mut file) = File::open(filename) {
        let mut buffer= Buffer::new();
        
        file.read_to_end(&mut buffer.buf);

        buffer.as_mut_ptr()
    } else {
        return null_mut()
    }
}

pub extern "C" fn ReadDumpWithMaxSize(filename: *const CStr, size: usize) -> *mut Buffer {}

pub extern "C" fn ReadDumpW(filename: *const i8) -> *mut Buffer {
    // Filename is made of wchar_t -- handle with rust properly
}

// pub extern "C" fn ReadDumpExW(buffer: *mut Buffer, out: *mut u8, size: usize) {

// }

// pub extern "C" fn ReadBufStr(buffer: *mut Buffer, out: *mut u8, size: usize) {

// }

pub extern "C" fn ReadRemainBuf(buffer: *mut Buffer) -> *mut Buffer {
    let buffer = unsafe { &mut *buffer };
}

pub extern "C" fn ReadBufRemainSize(buffer: *mut Buffer) -> u32 {
    let buffer = unsafe { &mut *buffer };
}

pub fn NewBuf() -> *mut Buffer {
    let buffer = Buffer::new();
    buffer.as_mut_ptr()
}
