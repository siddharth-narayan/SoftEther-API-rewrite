use std::net::{Ipv4Addr, Ipv6Addr};

use crate::mem::structs::buf::{Buffer};

static MAX_ELEMENT_NAME_LEN: usize = 64;

struct PackElement
{
	name: [u8; MAX_ELEMENT_NAME_LEN],
	num_value: usize,
	_type: u32,
	// VALUE **values;			
	// bool JsonHint_IsArray;
	// bool JsonHint_IsBool;
	// bool JsonHint_IsDateTime;
	// bool JsonHint_IsIP;
	// char JsonHint_GroupName[u8; MAX_ELEMENT_NAME_LEN];
}

struct Pack {
    _internal: Vec<PackElement>,
}

impl Pack {
    pub fn new() -> Self {
        Self { _internal: Vec::new() }
    }

    pub fn as_mut_ptr(self) -> *mut Self {
       Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Self) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn to_buf(self) -> Buffer {
        let mut out = Buffer::new();

        out.write_u32(self._internal.len() as u32);
        
        for element in self._internal.iter() {
            element.
        }

        out
    }
}


// PACK *NewPack()
// ELEMENT *GetElement(PACK*p,char*name,UINTtype)
// void FreePack(PACK*p)
// BUF *PackToBuf(PACK*p)
// PACK *BufToPack(BUF*b)
// X *PackGetX(PACK*p,char*name)
// LIST *PackGetXList(PACK*p,char*name)
// K *PackGetK(PACK*p,char*name)
// ELEMENT *PackAddX(PACK*p,char*name,X*x)
// ELEMENT *PackAddXList(PACK*p,char*name,LIST*chain)
// ELEMENT *PackAddK(PACK*p,char*name,K*k)
// ELEMENT *PackAddStr(PACK*p,char*name,char*str)
// ELEMENT *PackAddStrEx(PACK*p,char*name,char*str,UINTindex,UINTtotal)
// ELEMENT *PackAddUniStr(PACK*p,char*name,wchar_t*unistr)
// ELEMENT *PackAddUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTindex,UINTtotal)
// ELEMENT *PackAddInt(PACK*p,char*name,UINTi)
// ELEMENT *PackAddNum(PACK*p,char*name,UINTnum)
// ELEMENT *PackAddIntEx(PACK*p,char*name,UINTi,UINTindex,UINTtotal)
// ELEMENT *PackAddInt64(PACK*p,char*name,UINT64i)
// ELEMENT *PackAddInt64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
// ELEMENT *PackAddTime64(PACK*p,char*name,UINT64i)
// ELEMENT *PackAddTime64Ex(PACK*p,char*name,UINT64i,UINTindex,UINTtotal)
// ELEMENT *PackAddData(PACK*p,char*name,void*data,UINTsize)
// ELEMENT *PackAddDataEx(PACK*p,char*name,void*data,UINTsize,UINTindex,UINTtotal)
// ELEMENT *PackAddBuf(PACK*p,char*name,BUF*b)
// UINT PackGetStrSize(PACK*p,char*name)
// UINT PackGetStrSizeEx(PACK*p,char*name,UINTindex)
// bool PackGetStr(PACK*p,char*name,char*str,UINTsize)
// bool PackGetStrEx(PACK*p,char*name,char*str,UINTsize,UINTindex)
// bool PackGetUniStr(PACK*p,char*name,wchar_t*unistr,UINTsize)
// bool PackGetUniStrEx(PACK*p,char*name,wchar_t*unistr,UINTsize,UINTindex)
// UINT PackGetIndexCount(PACK*p,char*name)
// UINT PackGetInt(PACK*p,char*name)
// UINT PackGetNum(PACK*p,char*name)
// UINT PackGetIntEx(PACK*p,char*name,UINTindex)
// UINT64 PackGetInt64(PACK*p,char*name)
// UINT64 PackGetInt64Ex(PACK*p,char*name,UINTindex)
// UINT PackGetDataSizeEx(PACK*p,char*name,UINTindex)
// UINT PackGetDataSize(PACK*p,char*name)
// bool PackGetData(PACK*p,char*name,void*data)
// BUF *PackGetBuf(PACK*p,char*name)
// BUF *PackGetBufEx(PACK*p,char*name,UINTindex)
// bool PackGetBool(PACK*p,char*name)
// ELEMENT *PackAddBool(PACK*p,char*name,boolb)
// ELEMENT *PackAddBoolEx(PACK*p,char*name,boolb,UINTindex,UINTtotal)
// bool PackGetBoolEx(PACK*p,char*name,UINTindex)
// void PackAddIp(PACK*p,char*name,IP*ip)
// void PackAddIpEx(PACK*p,char*name,IP*ip,UINTindex,UINTtotal)
// bool PackGetIp(PACK*p,char*name,IP*ip)
// bool PackGetIpEx(PACK*p,char*name,IP*ip,UINTindex)
// UINT PackGetIp32(PACK*p,char*name)
// UINT PackGetIp32Ex(PACK*p,char*name,UINTindex)
// void PackAddIp32(PACK*p,char*name,UINTip32)
// void PackAddIp32Ex(PACK*p,char*name,UINTip32,UINTindex,UINTtotal)
// ELEMENT *PackAddIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex,UINTtotal)
// bool PackGetIp6AddrEx(PACK*p,char*name,IPV6_ADDR*addr,UINTindex)
// bool PackGetData2(PACK*p,char*name,void*data,UINTsize)
// bool PackGetDataEx2(PACK*p,char*name,void*data,UINTsize,UINTindex)
// bool PackIsValueExists(PACK*p,char*name)
// void PackSetCurrentJsonGroupName(PACK*p,char*json_group_name)
// JSON_VALUE *PackToJson(PACK*p)
// PACK *JsonToPack(JSON_VALUE*v)
