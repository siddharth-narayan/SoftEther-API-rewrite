use std::rc::Rc;
use std::{fs::File, io::Read};
use std::env;

use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::tree::ParseTree;

use crate::clexer::CLexer;
use crate::cparser::{CParser, CompilationUnitContextAttrs, DeclarationContext, DeclarationContextAttrs, DeclarationSpecifiersContextAttrs, DeclaratorContextAttrs, DirectDeclaratorContextAttrs, ExternalDeclarationContextAttrs, InitDeclaratorContextAttrs, InitDeclaratorListContextAttrs, PrimaryExpressionContextAttrs, TranslationUnitContextAttrs};

mod cparser;
mod clexer;
mod clistener;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).unwrap();

    // let mut file_content = String::new();
    let file_content = std::fs::read_to_string(path).unwrap();
    // File::open(path).unwrap().read_to_string(&mut file_content).unwrap();
    
    let lexer = CLexer::new(InputStream::new(file_content.as_str()));
    let mut parser = CParser::new(CommonTokenStream::new(lexer));

    if let Ok(result) = parser.compilationUnit() {
        let decls = result.translationUnit().unwrap().externalDeclaration_all();

        let mut iter = decls.iter().map(|ext_decl| { ext_decl.declaration().unwrap() });
        while let Some(decl) = iter.next() {
            if let Some(name) = process_ext_decl_to_func(decl) {
                println!("{}", name);
            }
        }
    }   
}

pub fn process_ext_decl_to_func(decl: Rc<DeclarationContext>) -> Option<String> {
    // This is the base function declaration
    let function_declaration = decl.initDeclaratorList()?
        .initDeclarator(0)?
        .declarator()?
        .directDeclarator()?;

    let name = function_declaration.directDeclarator()?.get_text();

    // Check if exits in symbols list
    if true {

    }
        // .directDeclarator()?
        // .get_text();

    Some(name)
}