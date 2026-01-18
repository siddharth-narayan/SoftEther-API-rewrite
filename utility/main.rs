use std::{fs::File, io::Read};

use crate::cparser::CParser;


mod cparser;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args[0];

    let mut file_content = String::new();
    File::open(path).unwrap().read_to_string(&mut file_content).unwrap();
    
    CParser::new(file_content)
}