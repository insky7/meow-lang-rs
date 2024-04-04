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


    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }
    fn advance(&mut self) -> Option<&Token> {
        // Check if there is a token to advance to
        if self.current < self.tokens.len() {
            // Temporarily store the current token to return it
            let token = &self.tokens[self.current];
            // Safely increment the current index now
            self.current += 1;
            // Return the token we advanced from
            Some(token)
        } else {
            // No more tokens to advance to
            None
        }
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        // Use self.advance() instead of self.current() to ensure the parser advances
        let token = self.advance()?;
        match token.kind {
            TokenKind::Number(number) => Some(ASTExpression::number(number)),
            _ => None,
        }
    }
}
