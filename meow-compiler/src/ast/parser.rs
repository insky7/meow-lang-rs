// src/ast/parser.rs
use crate::ast::lexer::{Lexer, Token, TokenKind}; 
use crate::ast::{ASTExpression, ASTStatement}; 

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self::new(tokens)
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        self.parse_statement()
    }

    pub fn parse_statement(&mut self) -> Option<ASTStatement> {
        self.parse_expression().map(ASTStatement::expression)
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        let token = self.current()?;
        match token.kind {
            TokenKind::Number(number) => Some(ASTExpression::number(number)),
            _ => None,
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }
}
