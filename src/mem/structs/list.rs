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

// StrList
// LIST *NewStrList();
// void ReleaseStrList(LIST *o);
// bool AddStrToStrListDistinct(LIST *o, char *str);