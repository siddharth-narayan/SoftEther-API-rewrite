use std::env;
use std::rc::Rc;

use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::parser_rule_context::BaseParserRuleContext;
use antlr4rust::tree::ParseTree;

use crate::clexer::CLexer;
use crate::cparser::{
    CParser, CompilationUnitContextAttrs,
    DeclarationContextAttrs, DeclaratorContextAttrs,
    DirectDeclaratorContextAttrs, ExternalDeclarationContextAll,
    ExternalDeclarationContextAttrs, ExternalDeclarationContextExt, InitDeclaratorContextAttrs,
    InitDeclaratorListContextAttrs, TranslationUnitContextAttrs,
};

mod clexer;
mod clistener;
mod cparser;

// long_name includes arguments etc
// (short_name, long_name)
type FunctionDesc = (String, String);

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let source_code = std::fs::read_to_string(args.get(1).unwrap()).unwrap();
    let symbols = std::fs::read_to_string(args.get(2).unwrap()).unwrap();
    let symbols: Vec<&str> = symbols.split("\n").collect();

    if let Some(contents) = parse_content(source_code.as_str()) {
        for desc in contents {
            if symbols.contains(&desc.0.as_str()) {
                println!("{}", desc.1);
            }
        }
    } else {
        println!("Some error occured :(");
    }
}

fn parse_content(content: &str) -> Option<Vec<FunctionDesc>> {
    let mut out: Vec<FunctionDesc> = Vec::new();

    let lexer = CLexer::new(InputStream::new(content));
    let mut parser = CParser::new(CommonTokenStream::new(lexer));

    let decls: Vec<Rc<BaseParserRuleContext<'_, ExternalDeclarationContextExt<'_>>>> = parser
        .compilationUnit()
        .ok()?
        .translationUnit()?
        .externalDeclaration_all();

    let mut iter = decls.into_iter();
    while let Some(decl) = iter.next() {
        if let Some(f_desc) = process_ext_decl_to_func(decl) {
            out.push(f_desc); // Switch to long_str when working
        };
    }

    Some(out)
}

pub fn process_ext_decl_to_func(
    ext_decl: Rc<ExternalDeclarationContextAll>,
) -> Option<FunctionDesc> {
    // This is the base function declaration
    let function_declaration = ext_decl
        .declaration()?
        .initDeclaratorList()?
        .initDeclarator(0)?
        .declarator()?
        .directDeclarator()?;

    let name = function_declaration.directDeclarator()?.get_text();
    Some((name, function_declaration.get_text()))
}
