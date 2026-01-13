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

use std::io::Write;


struct Buffer {
    buf: Vec<u8>,
    current_pos: usize
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buf: Vec::new(),
            current_pos: 0,
        }
    }

    pub fn as_mut_ptr(self) -> *mut Buffer {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }


}
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
impl Buffer {
    pub fn read(&mut self, data: &[u8]) {
        self.buf.read_all(data);
    }

    pub fn read_buf(&mut self, buffer: &Buffer) {
        self.buf.read_all(&buffer.clone().buf.as_slice());
    }

    pub fn read_buf_offset(&mut self, buffer: &Buffer) {
        self.buf.read_all(&buffer.clone().buf[buffer.current_pos..]);
    }

    pub fn read_u8(&mut self, char: u8) {
        self.buf.read()
    }

    pub fn read_u16(&mut self, int: u32) {
        self.buf.read()
    }

    pub fn read_u32(&mut self, int: u32) {
        self.buf.read()
    }

    pub fn read_u64(&mut self, int: u64 ) {
        self.buf.read()
    }

    pub fn read_str(&mut self, ) {
        self.buf.read()
    }

    pub fn read_line(&mut self, ) {
        self.buf.read()
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
        self.buf.write_all(&buffer.clone().buf.as_slice());
    }

    pub fn write_buf_offset(&mut self, buffer: &Buffer) {
        self.buf.write_all(&buffer.clone().buf[buffer.current_pos..]);
    }

    pub fn write_u8(&mut self, char: u8) {
        self.buf.write()
    }

    pub fn write_u16(&mut self, int: u32) {
        self.buf.write()
    }

    pub fn write_u32(&mut self, int: u32) {
        self.buf.write()
    }

    pub fn write_u64(&mut self, int: u64 ) {
        self.buf.write()
    }

    pub fn write_str(&mut self, ) {
        self.buf.write()
    }

    pub fn write_line(&mut self, ) {
        self.buf.write()
    }
}

pub fn NewBuf() -> *mut Buffer {
    let buffer = Buffer::new();
    buffer.as_mut_ptr()
}