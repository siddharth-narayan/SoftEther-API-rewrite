// PRAND *NewPRand(void *key, UINT key_size);
// void FreePRand(PRAND *r);
// void PRand(PRAND *p, void *data, UINT size);
// UINT PRandInt(PRAND *p);

// void *Base64FromBin(UINT *out_size, const void *src, const UINT size);
// void *Base64ToBin(UINT *out_size, const void *src, const UINT size);

// UINT Uncompress(void *dst, UINT dst_size, void *src, UINT src_size);
// UINT Compress(void *dst, UINT dst_size, void *src, UINT src_size);
// UINT CompressEx(void *dst, UINT dst_size, void *src, UINT src_size, UINT level);
// UINT CalcCompress(UINT src_size);
// BUF *CompressBuf(BUF *src_buf);
// BUF *UncompressBuf(BUF *src_buf);

// bool IsZero(void *data, UINT size);

// UINT SearchBin(void *data, UINT data_start, UINT data_size, void *key, UINT key_size);
// void CrashNow();
// UINT Power(UINT a, UINT b);

// void XorData(void *dst, void *src1, void *src2, UINT size);

#[unsafe(no_mangle)]
extern "C" fn XorData(destination: *mut u8, source_a: *mut u8, source_b: *mut u8, size: usize) {
    if destination.is_null() || source_a.is_null() || source_b.is_null() {
        return;
    }

    for index in 0..size {
        unsafe {
            *destination.add(index.into()) =
                *source_a.add(index.into()) ^ *source_b.add(index.into());
        }
    }
}