#![feature(default_field_values)]
#![feature(ip)]
#![allow(unused, non_snake_case)]
#![feature(box_as_ptr)]
#![feature(c_variadic)]
#![feature(string_into_chars)]

mod mem;

mod config;
mod dns;
mod kernel;
mod macros;
mod network;
mod object;
mod pack;
mod str;
mod table;
mod unix;
mod win32;

mod tests;
mod util;


