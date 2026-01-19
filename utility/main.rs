use std::{fs::File, io::Read};
use std::env;

use antlr_rust::InputStream;
use antlr_rust::token_stream::TokenStream;

use crate::clexer::CLexer;
use crate::cparser::CParser;

mod cparser;
mod clexer;
mod clistener;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args[0];

    let mut file_content = String::new();
    File::open(path).unwrap().read_to_string(&mut file_content).unwrap();
    
    let lexer = CLexer::new(InputStream::new(file_content));
    let parser = CParser::new(TokenStream::new(lexer));

    parser.

   
}