use std::ffi::{CStr, CString, c_char};

use widestring::{U16CStr, WideCStr};

pub unsafe fn clone_from_c_str(ptr: *const c_char) -> String {
    let str = unsafe { CStr::from_ptr(ptr) };
    str.to_string_lossy().to_string()
}

pub unsafe fn clone_from_uni_str(ptr: *const u16) -> String {
    let str = unsafe { U16CStr::from_ptr_str(ptr) };
    str.to_string_lossy()
}

pub fn url_decode(url: &CStr) -> String {
    let string = url.to_str().unwrap();

    let mut str_iter = string.as_bytes().iter();

    let mut final_string = String::new();

    while let Some(char) = str_iter.next() {
        if *char == b'%' && str_iter.len() >= 2 {
            let hex = vec![str_iter.next().unwrap(), str_iter.next().unwrap()];

            let c = u8::from_str_radix(format!("{}{}", hex[0], hex[1]).as_str(), 16).unwrap();

            let c = c as char;

            final_string.extend(std::iter::once(c));
        }

        if *char == b'+' {
            final_string.extend(std::iter::once(' '));
        }
    }

    final_string
}

// UINT StrLen(char*str)
// UINT StrSize(char*str)
// UINT StrCpy(char*dst,UINTsize,char*src)
// UINT StrCat(char*dst,UINTsize,char*src)
// UINT StrCatLeft(char*dst,UINTsize,char*src)
// char ToUpper(charc)
// void StrUpper(char*str)
// void StrLower(char*str)
// int StrCmp(char*str1,char*str2)
// int StrCmpi(char*str1,char*str2)
// void FormatArgs(char*buf,UINTsize,char*fmt,va_listargs)
// void Format(char*buf,UINTsize,char*fmt,...)
// void Print(char*fmt,...)
// void Debug(char*fmt,...)
// UINT ToInt(char*str)
// bool ToBool(char*str)
// void ToStr(char*str,UINTi)
// void TrimCrlf(char*str)
// void Trim(char*str)
// void FreeToken(TOKEN_LIST*tokens)
// TOKEN_LIST *ParseToken(char*src,char*separator)
// UINT SearchStrEx(char*string,char*keyword,UINTstart,boolcase_sensitive)
// UINT SearchStr(char*string,char*keyword,UINTstart)
// UINT ReplaceStrEx(char*dst,UINTsize,char*string,char*old_keyword,char*new_keyword,boolcase_sensitive)
// UINT ReplaceStr(char*dst,UINTsize,char*string,char*old_keyword,char*new_keyword)
// bool IsPrintableAsciiChar(charc)
// void EnPrintableAsciiStr(char*str,charreplace)
// bool IsSafeChar(charc)
// bool IsSafeStr(char*str)
// void EnSafeStr(char*str,charreplace)
// void EnSafeHttpHeaderValueStr(char*str,charreplace)
// char *CopyStr(char*str)
// void BinToStr(char*str,UINTstr_size,void*data,UINTdata_size)
// bool StartWith(char*str,char*key)
// bool EndWith(char*str,char*key)
// bool TrimEndWith(char*dst,UINTdst_size,char*str,char*key)
// TOKEN_LIST *NullToken()
// bool IsNum(char*str)
// void FreeStrList(LIST*o)
// TOKEN_LIST *ListToTokenList(LIST*o)
// bool IsEmptyStr(char*str)
// void BinToStrEx(char*str,UINTstr_size,void*data,UINTdata_size)
// char *CopyBinToStr(void*data,UINTdata_size)
// BUF *StrToBin(char*str)
// void MacToStr(char*str,UINTsize,UCHAR*mac_address)
// void ToStr3(char*str,UINTsize,UINT64v)
// void ToStrByte(char*str,UINTsize,UINT64v)
// void ToStrByte1000(char*str,UINTsize,UINT64v)
// bool IsAllUpperStr(char*str)
// UINT StrWidth(char*str)
// char *MakeCharArray(charc,UINTcount)
// void MakeCharArray2(char*str,charc,UINTcount)
// bool StrToMac(UCHAR*mac_address,char*str)
// bool GetKeyAndValue(char*str,char*key,UINTkey_size,char*value,UINTvalue_size,char*split_str)
// LIST *ReadIni(BUF*b)
// void FreeIni(LIST*o)
// char *IniStrValue(LIST*o,char*key)
// bool InStr(char*str,char*keyword)
// bool InStrEx(char*str,char*keyword,boolcase_sensitive)
// TOKEN_LIST *ParseTokenWithoutNullStr(char*str,char*split_chars)
// TOKEN_LIST *ParseTokenWithNullStr(char*str,char*split_chars)
// UINT HexToInt(char*str)
// void IntListToStr(char*str,UINTstr_size,LIST*o,char*separate_str)
// LIST *StrToIntList(char*str,boolsorted)
// void NormalizeIntListStr(char*dst,UINTdst_size,char*src,boolsorted,char*separate_str)
// void ClearStr(char*str,UINTstr_size)
// void SetStrCaseAccordingToBits(char*str,UINTbits)
// char *UrlDecode(char*url_str)
// char *JsonToStr(JSON_VALUE*v)
// JSON_VALUE *JsonGet(JSON_OBJECT*object,char*name)
// char *JsonGetStr(JSON_OBJECT*object,char*name)
// UINT JsonSet(JSON_OBJECT*object,char*name,JSON_VALUE*value)
// UINT JsonSetStr(JSON_OBJECT*object,char*name,char*string)
// UINT JsonSetUniStr(JSON_OBJECT*object,char*name,wchar_t*string)
// UINT JsonSetNumber(JSON_OBJECT*object,char*name,UINT64number)
// JSON_VALUE *JsonNewObject(void)
// void JsonFree(JSON_VALUE*value)
// JSON_OBJECT *JsonValueGetObject(JSON_VALUE*value)
// JSON_OBJECT *JsonObject(JSON_VALUE*value)
// JSON_VALUE *StrToJson(char*str)
