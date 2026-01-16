use std::{cmp::Ordering, ffi::c_void};

type CompareFunction<T> = Box<dyn for<'a, 'b>Fn(&'a T, &'b T) -> Ordering>;
type FfiCompareFunction = extern "C" fn(*const c_void, *const c_void ) -> i32;

struct List<T> {
    sorted: bool,
    items: Vec<T>,
    compare_function: CompareFunction<T>,
}



impl<T: Ord> List<T> {
    pub fn new() -> List<T> {
        List { sorted: true, items: Vec::new(), compare_function: Box::new(|item_1: &T, item_2: &T| {
            item_1.cmp(item_2)
        })}
    }
}

impl<T> List<T> {
    
    pub fn new_with_ffi_cmp(compare: FfiCompareFunction) -> List<T> {
        let rustified_compare_func = Box::new( move|item_1: &T, item_2: &T| {
            let (item_1, item_2) = (item_1 as *const T, item_2 as *const T);
            let (item_1, item_2) = (item_1 as *const c_void, item_2 as *const c_void);

            let result = (compare)(item_1, item_2);
            
            if result < 0 {
                std::cmp::Ordering::Less
            } else if result > 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        List { sorted: true, items: Vec::new(), compare_function: rustified_compare_func}
    }

    pub fn as_mut_ptr(self) -> *mut List<T> {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    pub fn free_mut_ptr(ptr: *mut List<T>) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn sort(&mut self) {
        self.items.sort_unstable_by(&self.compare_function)
    }

    pub fn search(&self, item: &T) -> Option<usize> {
        let search_closure = | cmp_item: &T | {
            (&self.compare_function)(item, cmp_item)
        };

        match self.items.binary_search_by(search_closure) {
            Ok(size) => Some(size),
            Err(_) => None,
        }
    }
}


// void *Search(LIST *o, void *target);
// void Sort(LIST *o);
// void Add(LIST *o, void *p);
// void AddDistinct(LIST *o, void *p);
// void Insert(LIST *o, void *p);
// bool Delete(LIST *o, void *p);
// void DeleteAll(LIST *o);
// void LockList(LIST *o);
// void UnlockList(LIST *o);
// void ReleaseList(LIST *o);
// void CleanupList(LIST *o);
// LIST *NewList(COMPARE *cmp);
// LIST *NewListFast(COMPARE *cmp);
// LIST *NewListEx(COMPARE *cmp, bool fast);
// LIST *NewListEx2(COMPARE *cmp, bool fast, bool fast_malloc);
// LIST *NewListSingle(void *p);
// LIST *NewEntryList(char *src, char *key_separator, char *value_separator);
// bool EntryListHasKey(LIST *o, char *key);
// char *EntryListStrValue(LIST *o, char *key);
// UINT EntryListIntValue(LIST *o, char *key);
// void FreeEntryList(LIST *o);
// LIST *CloneList(LIST *o);
// void CopyToArray(LIST *o, void *p);
// void *ToArray(LIST *o);
// void *ToArrayEx(LIST *o, bool fast);
// int CompareStr(void *p1, void *p2);
// bool InsertStr(LIST *o, char *str);
// int CompareUniStr(void *p1, void *p2);
// bool IsInList(LIST *o, void *p);
// bool IsInListKey(LIST *o, UINT key);
// void *ListKeyToPointer(LIST *o, UINT key);
// bool IsInListStr(LIST *o, char *str);
// bool IsInListUniStr(LIST *o, wchar_t *str);
// bool ReplaceListPointer(LIST *o, void *oldptr, void *newptr);
// void AddInt(LIST *o, UINT i);
// void AddInt64(LIST *o, UINT64 i);
// void AddIntDistinct(LIST *o, UINT i);
// void AddInt64Distinct(LIST *o, UINT64 i);
// void DelInt(LIST *o, UINT i);
// void ReleaseIntList(LIST *o);
// void ReleaseInt64List(LIST *o);
// bool IsIntInList(LIST *o, UINT i);
// bool IsInt64InList(LIST *o, UINT64 i);
// LIST *NewIntList(bool sorted);
// LIST *NewInt64List(bool sorted);
// int CompareInt(void *p1, void *p2);
// int CompareInt64(void *p1, void *p2);
// void InsertInt(LIST *o, UINT i);
// void InsertIntDistinct(LIST *o, UINT i);

// // Candidate
// LIST *NewCandidateList();
// void FreeCandidateList(LIST *o);
// int CompareCandidate(void *p1, void *p2);
// void AddCandidate(LIST *o, wchar_t *str, UINT num_max);
// BUF *CandidateToBuf(LIST *o);
// LIST *BufToCandidate(BUF *b);

// // Strmap
// LIST *NewStrMap();
// void *StrMapSearch(LIST *map, char *key);

// // HashList
// // Function prototype
// HASH_LIST *NewHashList(GET_HASH *get_hash_proc, COMPARE *compare_proc, UINT bits, bool make_list);
// void ReleaseHashList(HASH_LIST *h);
// void CleanupHashList(HASH_LIST *h);
// void AddHash(HASH_LIST *h, void *p);
// bool DeleteHash(HASH_LIST *h, void *p);
// void *SearchHash(HASH_LIST *h, void *t);
// UINT CalcHashForHashList(HASH_LIST *h, void *p);
// void **HashListToArray(HASH_LIST *h, UINT *num);
// void LockHashList(HASH_LIST *h);
// void UnlockHashList(HASH_LIST *h);
// bool IsInHashListKey(HASH_LIST *h, UINT key);
// void *HashListKeyToPointer(HASH_LIST *h, UINT key);

pub extern "C" fn Search(o: *mut List, target: *mut c_void) -> *mut c_void {}
pub extern "C" fn Sort(o: *mut List) {}
pub extern "C" fn Add(o: *mut List, p: *mut c_void) {}
pub extern "C" fn AddDistinct(o: *mut List, p: *mut c_void) {}
pub extern "C" fn Insert(o: *mut List, p: *mut c_void) {}
pub extern "C" fn Delete(o: *mut List, p: *mut c_void) -> bool {}
pub extern "C" fn DeleteAll(o: *mut List) {}
pub extern "C" fn LockList(o: *mut List) {}
pub extern "C" fn UnlockList(o: *mut List) {}
pub extern "C" fn ReleaseList(o: *mut List) {}
pub extern "C" fn CleanupList(o: *mut List) {}

pub extern "C" fn NewList(cmp: Compare) -> *mut List {}
pub extern "C" fn NewListFast(cmp: Compare) -> *mut List {}
pub extern "C" fn NewListEx(cmp: Compare, fast: bool) -> *mut List {}
pub extern "C" fn NewListEx2(cmp: Compare, fast: bool, fast_malloc: bool) -> *mut List {}
pub extern "C" fn NewListSingle(p: *mut c_void) -> *mut List {}
pub extern "C" fn NewEntryList(src: *const c_char, key_separator: *const c_char, value_separator: *const c_char) -> *mut List {}
pub extern "C" fn EntryListHasKey(o: *mut List, key: *const c_char) -> bool {}
pub extern "C" fn EntryListStrValue(o: *mut List, key: *const c_char) -> *mut c_char {}
pub extern "C" fn EntryListIntValue(o: *mut List, key: *const c_char) -> u32 {}
pub extern "C" fn FreeEntryList(o: *mut List) {}
pub extern "C" fn CloneList(o: *mut List) -> *mut List {}
pub extern "C" fn CopyToArray(o: *mut List, p: *mut c_void) {}
pub extern "C" fn ToArray(o: *mut List) -> *mut c_void {}
pub extern "C" fn ToArrayEx(o: *mut List, fast: bool) -> *mut c_void {}

pub extern "C" fn CompareStr(p1: *mut c_void, p2: *mut c_void) -> i32 {}
pub extern "C" fn InsertStr(o: *mut List, str: *const c_char) -> bool {}
pub extern "C" fn CompareUniStr(p1: *mut c_void, p2: *mut c_void) -> i32 {}

pub extern "C" fn IsInList(o: *mut List, p: *mut c_void) -> bool {}
pub extern "C" fn IsInListKey(o: *mut List, key: u32) -> bool {}
pub extern "C" fn ListKeyToPointer(o: *mut List, key: u32) -> *mut c_void {}
pub extern "C" fn IsInListStr(o: *mut List, str: *const c_char) -> bool {}
pub extern "C" fn IsInListUniStr(o: *mut List, str: *const u16) -> bool {}
pub extern "C" fn ReplaceListPointer(o: *mut List, oldptr: *mut c_void, newptr: *mut c_void) -> bool {}

pub extern "C" fn AddInt(o: *mut List, i: u32) {}
pub extern "C" fn AddInt64(o: *mut List, i: u64) {}
pub extern "C" fn AddIntDistinct(o: *mut List, i: u32) {}
pub extern "C" fn AddInt64Distinct(o: *mut List, i: u64) {}
pub extern "C" fn DelInt(o: *mut List, i: u32) {}
pub extern "C" fn ReleaseIntList(o: *mut List) {}
pub extern "C" fn ReleaseInt64List(o: *mut List) {}
pub extern "C" fn IsIntInList(o: *mut List, i: u32) -> bool {}
pub extern "C" fn IsInt64InList(o: *mut List, i: u64) -> bool {}
pub extern "C" fn NewIntList(sorted: bool) -> *mut List {}
pub extern "C" fn NewInt64List(sorted: bool) -> *mut List {}

pub extern "C" fn CompareInt(p1: *mut c_void, p2: *mut c_void) -> i32 {}
pub extern "C" fn CompareInt64(p1: *mut c_void, p2: *mut c_void) -> i32 {}
pub extern "C" fn InsertInt(o: *mut List, i: u32) {}
pub extern "C" fn InsertIntDistinct(o: *mut List, i: u32) {}

// Candidate
pub extern "C" fn NewCandidateList() -> *mut List {}
pub extern "C" fn FreeCandidateList(o: *mut List) {}
pub extern "C" fn CompareCandidate(p1: *mut c_void, p2: *mut c_void) -> i32 {}
pub extern "C" fn AddCandidate(o: *mut List, str: *const u16, num_max: u32) {}
pub extern "C" fn CandidateToBuf(o: *mut List) -> *mut Buffer {}
pub extern "C" fn BufToCandidate(b: *mut Buffer) -> *mut List {}

// Strmap
pub extern "C" fn NewStrMap() -> *mut List {}
pub extern "C" fn StrMapSearch(map: *mut List, key: *const c_char) -> *mut c_void {}

// HashList
pub extern "C" fn NewHashList(get_hash_proc: GetHash, compare_proc: Compare, bits: u32, make_list: bool) -> *mut HashList {}
pub extern "C" fn ReleaseHashList(h: *mut HashList) {}
pub extern "C" fn CleanupHashList(h: *mut HashList) {}
pub extern "C" fn AddHash(h: *mut HashList, p: *mut c_void) {}
pub extern "C" fn DeleteHash(h: *mut HashList, p: *mut c_void) -> bool {}
pub extern "C" fn SearchHash(h: *mut HashList, t: *mut c_void) -> *mut c_void {}
pub extern "C" fn CalcHashForHashList(h: *mut HashList, p: *mut c_void) -> u32 {}
pub extern "C" fn HashListToArray(h: *mut HashList, num: *mut u32) -> *mut *mut c_void {}
pub extern "C" fn LockHashList(h: *mut HashList) {}
pub extern "C" fn UnlockHashList(h: *mut HashList) {}
pub extern "C" fn IsInHashListKey(h: *mut HashList, key: u32) -> bool {}
pub extern "C" fn HashListKeyToPointer(h: *mut HashList, key: u32) -> *mut c_void {}

// StrList
// LIST *NewStrList();
// void ReleaseStrList(LIST *o);
// bool AddStrToStrListDistinct(LIST *o, char *str);