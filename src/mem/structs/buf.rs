// BUF *NewBuf();
// BUF *NewBufFromMemory(void *buf, UINT size);
// void ClearBuf(BUF *b);
// void WriteBuf(BUF *b, void *buf, UINT size);
// void WriteBufBuf(BUF *b, BUF *bb);
// void WriteBufBufWithOffset(BUF *b, BUF *bb);
// UINT ReadBuf(BUF *b, void *buf, UINT size);
// bool BufSkipUtf8Bom(BUF *b);
// BUF *ReadBufFromBuf(BUF *b, UINT size);
// void AdjustBufSize(BUF *b, UINT new_size);
// void SeekBuf(BUF *b, UINT offset, int mode);
// void SeekBufToEnd(BUF *b);
// void SeekBufToBegin(BUF *b);
// void FreeBuf(BUF *b);
// bool BufToFile(IO *o, BUF *b);
// BUF *FileToBuf(IO *o);
// UINT ReadBufInt(BUF *b);
// USHORT ReadBufShort(BUF *b);
// UINT64 ReadBufInt64(BUF *b);
// UCHAR ReadBufChar(BUF *b);
// bool WriteBufInt(BUF *b, UINT value);
// bool WriteBufInt64(BUF *b, UINT64 value);
// bool WriteBufChar(BUF *b, UCHAR uc);
// bool WriteBufShort(BUF *b, USHORT value);
// bool ReadBufStr(BUF *b, char *str, UINT size);
// bool WriteBufStr(BUF *b, char *str);
// void WriteBufLine(BUF *b, char *str);
// void AddBufStr(BUF *b, char *str);
// bool DumpBuf(BUF *b, char *filename);
// bool DumpBufW(BUF *b, wchar_t *filename);
// bool DumpBufWIfNecessary(BUF *b, wchar_t *filename);
// bool DumpDataW(void *data, UINT size, wchar_t *filename);
// BUF *ReadDump(char *filename);
// BUF *ReadDumpWithMaxSize(char *filename, UINT max_size);
// BUF *ReadDumpW(wchar_t *filename);
// BUF *ReadDumpExW(wchar_t *filename, bool read_lock);
// BUF *CloneBuf(BUF *b);
// BUF *MemToBuf(void *data, UINT size);
// BUF *RandBuf(UINT size);
// BUF *ReadRemainBuf(BUF *b);
// UINT ReadBufRemainSize(BUF *b);
// bool CompareBuf(BUF *b1, BUF *b2);

// SHARED_BUFFER *NewSharedBuffer(void *data, UINT size);
// void ReleaseSharedBuffer(SHARED_BUFFER *b);
// void CleanupSharedBuffer(SHARED_BUFFER *b);

// void AppendBufUtf8(BUF *b, wchar_t *str);
// void AppendBufStr(BUF *b, char *str);


struct Buffer<T> {
    buf: VecDeque<T>,
    current_pos: usize
}

impl Buffer<T> {
    pub fn new() -> Buffer<T> {
        {
            buf: VecDeque::new::<T>(),
            current_pos: 0,
        }
    }
}

pub fn NewBuf<T>() -> *mut Buffer<T> {
    let buffer = Buffer<T>::new()
    unsafe { buffer.as_mut_ptr() }
}