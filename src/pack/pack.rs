use std::{
    cmp::Ordering, net::{Ipv4Addr, Ipv6Addr}, os::raw::c_void, ptr::null_mut
};

use crate::mem::structs::buf::Buffer;
use crate::mem::structs::list::List;

static MAX_ELEMENT_NAME_LEN: usize = 64;

// VALUE object
#[derive(Clone)]
pub struct PackInnerValue {
    size: u32,
    int_val: u32,
    data: *mut c_void,
    str: *const u8,
    uni_str: *const u16,
    u64_val: u64,
}

impl PackInnerValue {
    pub fn new() -> Self {
        Self {
            size: 0,
            int_val: 0,
            data: null_mut(),
            str: null_mut(),
            uni_str: null_mut(),
            u64_val: 0
        }
    }

    // Placeholder
    pub fn str(&self) -> String {
        String::new()
    }

    pub fn u64(&self) -> u64 {
        0
    }

    pub fn bool(&self) -> bool {
        false
    }

    pub fn buf(&mut self) -> &mut Buffer {
        todo!()
    }
}

#[derive(Clone)]
enum PackElementType {}

#[repr(C)]
#[derive(Clone)]
pub struct PackElement {
    name: [u8; MAX_ELEMENT_NAME_LEN],
    num_value: usize,
    __type: u32,
    values_ptr: *const *mut PackInnerValue,
    json_is_array: bool,
    json_isbool: bool,
    json_is_datetime: bool,
    json_is_ip: bool,
    json_groupname: [u8; MAX_ELEMENT_NAME_LEN],

    // Rust internals
    _name: String,
    _type: PackElementType,
    values: Vec<PackInnerValue>,
}

impl PartialEq for PackElement {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for PackElement {}

impl PartialOrd for PackElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..MAX_ELEMENT_NAME_LEN {
            if self.name[i] > other.name[i] {
                return std::cmp::Ordering::Greater;
            }
            if self.name[i] > other.name[i] {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PackElement {
    pub fn get(&self, index: usize) -> Option<&PackInnerValue> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut PackInnerValue> {
        self.values.get_mut(index)
    }

    fn into_bytes(self) {}
}

#[repr(C)]
#[derive(Clone)]
pub struct Pack {
    elements: *const List<PackElement>,
    json_subitem_names: *mut List<String>, // String or something else?
    json_groupname: [u8; MAX_ELEMENT_NAME_LEN],

    _internal: List<PackElement>,
}

impl Pack {
    pub fn new() -> Self {
        Self {
            elements: null_mut(),
            json_subitem_names: null_mut(),
            json_groupname: [0; MAX_ELEMENT_NAME_LEN],

            _internal: List::new(),
        }
    }

    pub fn as_mut_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn free_mut_ptr(ptr: *mut Self) {
        unsafe { drop(Box::from_raw(ptr)) }
    }

    pub fn get_element(&mut self, name: String) -> Option<&mut PackElement> {
        for item in self._internal.iter_mut() {
            if item._name.cmp(&name) == Ordering::Equal {
                return Some(item)
            }
        }

        None
    }

    pub fn get_value(&mut self, name: String, index: u32) -> Option<&mut PackInnerValue> {
        let element = self.get_element(name);
        match element {
            Some(e) => {
                e.values.get_mut(index as usize)
            },
            None => None
        }
    }

    pub fn to_buf(self) -> Buffer {
        let mut out = Buffer::new();

        out.write_u32(self._internal.len() as u32);

        for element in self._internal.into_iter() {
            element.into_bytes()
        }

        out
    }

    pub fn from_buf(buf: Buffer) -> Self {
        Pack {
            elements: null_mut(),
            json_subitem_names: null_mut(),
            json_groupname: [0; MAX_ELEMENT_NAME_LEN],
            _internal: List::new(),
        };

        todo!()
    }
}
