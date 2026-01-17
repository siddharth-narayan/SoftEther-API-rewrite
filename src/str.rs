use std::ffi::{CStr, CString, c_char};

pub fn url_decode(url: &CStr) -> CString {
    let string = url.to_str().unwrap();

    let str_iter = string.as_bytes().iter();
	
    let mut final_string = String::new();

    while let Some(char) = str_iter.next() {
        if *char == b'%' && str_iter.len() >= 2 {
            let hex: Vec<&u8> = str_iter.take(2).collect();

            let c = u8::from_str_radix(format!("{}{}", hex[0], hex[1]).as_str(), 16).unwrap();

            let c = c as char;
            
            final_string.extend(std::iter::once(c));
            str_iter.skip(2);
        }

        if *char == b'+' {
            final_string.extend(std::iter::once(' '));
        }
    }
    
    final_string
}

// pub extern "C" fn UrlDecode(url: *const c_char) -> *mut c_char {
//     url_decode(url).into_raw()
// }