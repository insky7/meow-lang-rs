mod ast;
use crate::ast::{Ast, MyASTVisitor}; 
use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;

fn main() {
    let input: &str = "4";
    let mut visitor = MyASTVisitor::new();
    let mut lexer = Lexer::new(input); 
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("Tokens: {:?}", tokens);
    let mut parser = Parser::from_input(input); 
    let mut ast = Ast::new(); 
    //while let Some(next_statement) = parser.parse_statement() {
    //    ast.add_statement(next_statement);
    //}
    ast.visit(&mut visitor);
}
