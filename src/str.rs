use std::ffi::{CStr, CString, c_char};

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

// StrLen(char*str)
// StrSize(char*str)
// StrCpy(char*dst,UINTsize,char*src)
// StrCat(char*dst,UINTsize,char*src)
// StrCatLeft(char*dst,UINTsize,char*src)
// ToUpper(charc)
// StrUpper(char*str)
// StrLower(char*str)
// StrCmp(char*str1,char*str2)
// StrCmpi(char*str1,char*str2)
// FormatArgs(char*buf,UINTsize,char*fmt,va_listargs)
// Format(char*buf,UINTsize,char*fmt,...)
// Print(char*fmt,...)
// Debug(char*fmt,...)
// ToInt(char*str)
// ToBool(char*str)
// ToStr(char*str,UINTi)
// TrimCrlf(char*str)
// Trim(char*str)
// FreeToken(TOKEN_LIST*tokens)
// ParseToken(char*src,char*separator)
// SearchStrEx(char*string,char*keyword,UINTstart,boolcase_sensitive)
// SearchStr(char*string,char*keyword,UINTstart)
// ReplaceStrEx(char*dst,UINTsize,char*string,char*old_keyword,char*new_keyword,boolcase_sensitive)
// ReplaceStr(char*dst,UINTsize,char*string,char*old_keyword,char*new_keyword)
// IsPrintableAsciiChar(charc)
// EnPrintableAsciiStr(char*str,charreplace)
// IsSafeChar(charc)
// IsSafeStr(char*str)
// EnSafeStr(char*str,charreplace)
// EnSafeHttpHeaderValueStr(char*str,charreplace)
// CopyStr(char*str)
// BinToStr(char*str,UINTstr_size,void*data,UINTdata_size)
// StartWith(char*str,char*key)
// EndWith(char*str,char*key)
// TrimEndWith(char*dst,UINTdst_size,char*str,char*key)
// NullToken()
// IsNum(char*str)
// FreeStrList(LIST*o)
// ListToTokenList(LIST*o)
// IsEmptyStr(char*str)
// BinToStrEx(char*str,UINTstr_size,void*data,UINTdata_size)
// CopyBinToStr(void*data,UINTdata_size)
// StrToBin(char*str)
// MacToStr(char*str,UINTsize,UCHAR*mac_address)
// ToStr3(char*str,UINTsize,UINT64v)
// ToStrByte(char*str,UINTsize,UINT64v)
// ToStrByte1000(char*str,UINTsize,UINT64v)
// IsAllUpperStr(char*str)
// StrWidth(char*str)
// MakeCharArray(charc,UINTcount)
// MakeCharArray2(char*str,charc,UINTcount)
// StrToMac(UCHAR*mac_address,char*str)
// GetKeyAndValue(char*str,char*key,UINTkey_size,char*value,UINTvalue_size,char*split_str)
// ReadIni(BUF*b)
// FreeIni(LIST*o)
// IniStrValue(LIST*o,char*key)
// InStr(char*str,char*keyword)
// InStrEx(char*str,char*keyword,boolcase_sensitive)
// ParseTokenWithoutNullStr(char*str,char*split_chars)
// ParseTokenWithNullStr(char*str,char*split_chars)
// HexToInt(char*str)
// IntListToStr(char*str,UINTstr_size,LIST*o,char*separate_str)
// StrToIntList(char*str,boolsorted)
// NormalizeIntListStr(char*dst,UINTdst_size,char*src,boolsorted,char*separate_str)
// ClearStr(char*str,UINTstr_size)
// SetStrCaseAccordingToBits(char*str,UINTbits)
// UrlDecode(char*url_str)
// JsonToStr(JSON_VALUE*v)
// JsonGet(JSON_OBJECT*object,char*name)
// JsonGetStr(JSON_OBJECT*object,char*name)
// JsonSet(JSON_OBJECT*object,char*name,JSON_VALUE*value)
// JsonSetStr(JSON_OBJECT*object,char*name,char*string)
// JsonSetUniStr(JSON_OBJECT*object,char*name,wchar_t*string)
// JsonSetNumber(JSON_OBJECT*object,char*name,UINT64number)
// JsonNewObject(void)
// JsonFree(JSON_VALUE*value)
// JsonValueGetObject(JSON_VALUE*value)
// JsonObject(JSON_VALUE*value)
// StrToJson(char*str)
