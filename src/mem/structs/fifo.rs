// UINT ReadFifo(FIFO *f, void *p, UINT size);
// BUF *ReadFifoAll(FIFO *f);
// void ShrinkFifoMemory(FIFO *f);
// UCHAR *GetFifoPointer(FIFO *f); // Not exported?
// UCHAR *FifoPtr(FIFO *f);
// void WriteFifo(FIFO *f, void *p, UINT size);
// UINT FifoSize(FIFO *f);
// void ReleaseFifo(FIFO *f);
// void CleanupFifo(FIFO *f); // Not exported?
// FIFO *NewFifo();
// FIFO *NewFifoFast();
// FIFO *NewFifoEx(bool fast); // Not exported?
// FIFO *NewFifoEx2(bool fast, bool fixed); // Not exported?
// void InitFifo(); // Not exported?
// void SetFifoCurrentReallocMemSize(UINT size);

// pub extern "C" fn ReadFifo(fifo: *mut Fifo, out: *mut u8, size: usize) {}
// pub extern "C" fn ReadFifo(fifo: *mut Fifo) {}
// pub extern "C" fn ReadFifo(fifo: *mut Fifo) {}
// pub extern "C" fn ReadFifo(fifo: *mut Fifo) {}
// pub extern "C" fn WriteFifo(fifo: *mut Fifo, out: *mut u8, size: usize) {}
// pub extern "C" fn FifoSize(fifo: *mut Fifo) {}
// pub extern "C" fn ReleaseFifo(fifo: *mut Fifo) {}
// pub extern "C" fn NewFifo() {}
// pub extern "C" fn NewFifoFast() {}
// pub extern "C" fn SetFifoCurrentReallocMemSize(size: usize) {
//     // Can ignore for now?
// }
