use std::ffi::{CStr, CString, c_char};

// pub fn url_decode(url: &CStr) -> CString {
//     url.to_str().unwrap()

// 	for i in 0..len {
		
// 	}


// 	for (i = 0;i < len;i++)
// 	{
// 		char c = url_str[i];

// 		if (c == '%' && ((i + 2) < len))
// 		{
// 			char hex_str[8];
// 			UINT value;

// 			hex_str[0] = url_str[i + 1];
// 			hex_str[1] = url_str[i + 2];
// 			hex_str[2] = 0;

// 			value = HexToInt(hex_str);

// 			WriteBufChar(b, (UCHAR)value);

// 			i += 2;
// 			continue;
// 		}
// 		else
// 		{
// 			if (c == '+')
// 			{
// 				c = ' ';
// 			}
// 			WriteBufChar(b, c);
// 		}
// 	}

// 	WriteBufChar(b, 0);

// 	ret = CopyStr(b->Buf);

// 	FreeBuf(b);

// 	return ret;
// }

// pub extern "C" fn UrlDecode(url: *const c_char) -> *mut c_char {
//     url_decode(url).into_raw()
// }