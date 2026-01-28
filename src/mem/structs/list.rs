use std::{cmp::Ordering, ffi::c_void, ptr::null_mut, vec::IntoIter};

use crate::{
    object::{Lock, RefCounter},
    util::RawPtr,
};

type CompareFunction<T> = Box<dyn for<'a, 'b> Fn(&'a T, &'b T) -> Ordering>;
pub type FfiCompareFunction = extern "C" fn(*const c_void, *const c_void) -> i32;

#[repr(C)]
pub struct List<T> {
    ref_count: *mut RefCounter,
    num_items: u32,
    num_reserved: u32,
    items_ptr: *const *mut c_void,
    lock: *mut Lock,
    cmp: Option<FfiCompareFunction>, // None functions as NULL

    // Rust internals
    sorted: bool,
    __param: u64, // ?? What does this do
    items: Vec<T>,
    compare_function: CompareFunction<T>,
}

impl<T: Ord> List<T> {
    pub fn new() -> List<T> {
        List {
            ref_count: null_mut(),
            num_items: 0,
            num_reserved: 0,
            items_ptr: null_mut(),
            lock: null_mut(),
            cmp: None,
            __param: 0,

            sorted: true,
            items: Vec::new(),
            compare_function: Box::new(|item_1: &T, item_2: &T| item_1.cmp(item_2)),
        }
    }
}

impl<T> List<T> {
    pub fn new_with_ffi_cmp(compare: FfiCompareFunction) -> List<T> {
        let rustified_compare_func = Box::new(move |item_1: &T, item_2: &T| {
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

        List {
            ref_count: null_mut(),
            num_items: 0,
            num_reserved: 0,
            items_ptr: null_mut(),
            lock: null_mut(),
            cmp: unsafe { std::mem::transmute(compare) },
            __param: 0,

            sorted: true,
            items: Vec::new(),
            compare_function: rustified_compare_func,
        }
    }

    pub fn as_mut_ptr(self) -> *mut List<T> {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut List<T>) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }

    pub fn into_iter(self) -> IntoIter<T> {
        self.items.into_iter()
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn delete(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn get(&mut self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn sort(&mut self) {
        self.items.sort_unstable_by(&self.compare_function)
    }

    pub fn search(&self, item: &T) -> Option<usize> {
        let search_closure = |cmp_item: &T| (&self.compare_function)(item, cmp_item);

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

#[unsafe(no_mangle)]
pub extern "C" fn Search(ptr: *mut List<RawPtr>, target: RawPtr) -> RawPtr {
    let list = unsafe { &mut *ptr };

    if let Some(idx) = list.search(&target) {
        return list.get(idx).unwrap_or(&null_mut()).clone();
    }

    return null_mut();
}

#[unsafe(no_mangle)]
pub extern "C" fn Sort(ptr: *mut List<RawPtr>) {
    let list = unsafe { &mut *ptr };

    list.sort();
}

#[unsafe(no_mangle)]
pub extern "C" fn Add(ptr: *mut List<RawPtr>, value: RawPtr) {
    let list = unsafe { &mut *ptr };

    list.add(value);
}

#[unsafe(no_mangle)]
pub extern "C" fn AddDistinct(ptr: *mut List<RawPtr>, item: RawPtr) {
    let list = unsafe { &mut *ptr };

    if None == list.search(&item) {
        list.add(item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Insert(ptr: *mut List<RawPtr>, item: RawPtr) {}

#[unsafe(no_mangle)]
pub extern "C" fn Delete(ptr: *mut List<RawPtr>, item: RawPtr) -> bool {
    let list = unsafe { &mut *ptr };

    if let Some(index) = list.search(&item) {
        list.delete(index);

        return true;
    }

    return false;
}

#[unsafe(no_mangle)]
pub extern "C" fn DeleteAll(ptr: *mut List<RawPtr>) {
    let list = unsafe { &mut *ptr };

    list.items.clear();
}

#[unsafe(no_mangle)]
pub extern "C" fn LockList(ptr: *mut List<RawPtr>) {}

#[unsafe(no_mangle)]
pub extern "C" fn UnlockList(ptr: *mut List<RawPtr>) {}

#[unsafe(no_mangle)]
pub extern "C" fn ReleaseList(ptr: *mut List<RawPtr>) {
    List::free_mut_ptr(ptr);
}

// UNUSED
// pub extern "C" fn CleanupList(ptr: *mut List) {}

#[unsafe(no_mangle)]
pub extern "C" fn NewList(cmp: FfiCompareFunction) -> *mut List<RawPtr> {
    List::new().as_mut_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn NewListFast(cmp: FfiCompareFunction) -> *mut List<RawPtr> {
    NewList(cmp)
}

// UNUSED
// pub extern "C" fn NewListEx(cmp: Compare, fast: bool) -> *mut List {}

// UNUSED
// pub extern "C" fn NewListEx2(cmp: Compare, fast: bool, fast_malloc: bool) -> *mut List {}

#[unsafe(no_mangle)]
pub extern "C" fn NewListSingle(p: RawPtr) -> *mut List<RawPtr> {
    let mut l = List::new();
    l.add(p);

    l.as_mut_ptr()
}

// ALL THIS JUST COMMENTED OUT FOR NOW

// pub extern "C" fn NewEntryList(src: *const c_char, key_separator: *const c_char, value_separator: *const c_char) -> *mut List {}

// pub extern "C" fn EntryListHasKey(ptr: *mut List, key: *const c_char) -> bool {}

// pub extern "C" fn EntryListStrValue(ptr: *mut List, key: *const c_char) -> *mut c_char {}

// pub extern "C" fn EntryListIntValue(ptr: *mut List, key: *const c_char) -> u32 {}

// pub extern "C" fn FreeEntryList(ptr: *mut List) {}

// pub extern "C" fn CloneList(ptr: *mut List) -> *mut List {}

// // pub extern "C" fn CopyToArray(ptr: *mut List, p: RawPtr) {}

// pub extern "C" fn ToArray(ptr: *mut List) -> RawPtr {}

// pub extern "C" fn ToArrayEx(ptr: *mut List, fast: bool) -> RawPtr {}

// pub extern "C" fn CompareStr(p1: RawPtr, p2: RawPtr) -> i32 {}

// pub extern "C" fn InsertStr(ptr: *mut List, str: *const c_char) -> bool {}

// pub extern "C" fn CompareUniStr(p1: RawPtr, p2: RawPtr) -> i32 {}

// pub extern "C" fn IsInList(ptr: *mut List, p: RawPtr) -> bool {}

// pub extern "C" fn IsInListKey(ptr: *mut List, key: u32) -> bool {}

// pub extern "C" fn ListKeyToPointer(ptr: *mut List, key: u32) -> RawPtr {}

// pub extern "C" fn IsInListStr(ptr: *mut List, str: *const c_char) -> bool {}

// pub extern "C" fn IsInListUniStr(ptr: *mut List, str: *const u16) -> bool {}

// pub extern "C" fn ReplaceListPointer(ptr: *mut List, oldptr: RawPtr, newptr: RawPtr) -> bool {}

// pub extern "C" fn AddInt(ptr: *mut List, i: u32) {}

// pub extern "C" fn AddInt64(ptr: *mut List, i: u64) {}

// pub extern "C" fn AddIntDistinct(ptr: *mut List, i: u32) {}

// pub extern "C" fn AddInt64Distinct(ptr: *mut List, i: u64) {}

// pub extern "C" fn DelInt(ptr: *mut List, i: u32) {}

// pub extern "C" fn ReleaseIntList(ptr: *mut List) {}

// pub extern "C" fn ReleaseInt64List(ptr: *mut List) {}

// pub extern "C" fn IsIntInList(ptr: *mut List, i: u32) -> bool {}

// pub extern "C" fn IsInt64InList(ptr: *mut List, i: u64) -> bool {}

// pub extern "C" fn NewIntList(sorted: bool) -> *mut List {}

// pub extern "C" fn NewInt64List(sorted: bool) -> *mut List {}

// pub extern "C" fn CompareInt(p1: RawPtr, p2: RawPtr) -> i32 {}

// pub extern "C" fn CompareInt64(p1: RawPtr, p2: RawPtr) -> i32 {}

// pub extern "C" fn InsertInt(ptr: *mut List, i: u32) {}

// pub extern "C" fn InsertIntDistinct(ptr: *mut List, i: u32) {}

// // Candidate

// pub extern "C" fn NewCandidateList() -> *mut List {}

// pub extern "C" fn FreeCandidateList(ptr: *mut List) {}

// pub extern "C" fn CompareCandidate(p1: RawPtr, p2: RawPtr) -> i32 {}

// pub extern "C" fn AddCandidate(ptr: *mut List, str: *const u16, num_max: u32) {}

// pub extern "C" fn CandidateToBuf(ptr: *mut List) -> *mut Buffer {}

// pub extern "C" fn BufToCandidate(b: *mut Buffer) -> *mut List {}

// // Strmap

// pub extern "C" fn NewStrMap() -> *mut List {}

// pub extern "C" fn StrMapSearch(map: *mut List, key: *const c_char) -> RawPtr {}

// // HashList

// pub extern "C" fn NewHashList(get_hash_proc: GetHash, compare_proc: Compare, bits: u32, make_list: bool) -> *mut List<Hash> {}

// pub extern "C" fn ReleaseHashList(h: *mut List<Hash>) {}

// pub extern "C" fn CleanupHashList(h: *mut List<Hash>) {}

// pub extern "C" fn AddHash(h: *mut List<Hash>, p: RawPtr) {}

// pub extern "C" fn DeleteHash(h: *mut List<Hash>, p: RawPtr) -> bool {}

// pub extern "C" fn SearchHash(h: *mut List<Hash>, t: RawPtr) -> RawPtr {}

// pub extern "C" fn CalcHashForHashList(h: *mut List<Hash>, p: RawPtr) -> u32 {}

// pub extern "C" fn HashListToArray(h: *mut List<Hash>, num: *mut u32) -> *mut RawPtr {}

// pub extern "C" fn LockHashList(h: *mut List<Hash>) {}

// pub extern "C" fn UnlockHashList(h: *mut List<Hash>) {}

// pub extern "C" fn IsInHashListKey(h: *mut List<Hash>, key: u32) -> bool {}

// pub extern "C" fn HashListKeyToPointer(h: *mut List<Hash>, key: u32) -> RawPtr {}

// StrList

// LIST *NewStrList();

// void ReleaseStrList(LIST *o);

// bool AddStrToStrListDistinct(LIST *o, char *str);
