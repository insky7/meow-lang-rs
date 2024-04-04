// src/ast/lexer.rs

#[derive(Debug)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Bad(String),
    Eof,
}

#[derive(Debug)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        TextSpan { start, end }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    eof_reached: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
            eof_reached: false,
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        if self.eof_reached {
            return None;
        }
        if self.current_pos >= self.input.len() {
            self.eof_reached = true; // Mark EOF as reached
            return Some(Token::new(TokenKind::Eof, TextSpan::new(self.current_pos, self.current_pos)));
        }

        let start = self.current_pos;
        let c = self.current_char();

        let kind = match c {
            '0'..='9' => {
                let number = self.eat_num();
                TokenKind::Number(number)
            },
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => {
                let unrecognized = c.to_string();
                self.eat_char(); // Ensure you move past the bad character
                TokenKind::Bad(unrecognized)
            },
        };

        let end = self.current_pos;

        Some(Token::new(kind, TextSpan::new(start, end)))
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap_or('\0')
    }

    fn eat_char(&mut self) {
        if self.current_pos < self.input.len() {
            self.current_pos += 1;
        }
    }

    fn eat_num(&mut self) -> i64 {
        let start = self.current_pos;
        while let Some(c) = self.input.chars().nth(self.current_pos) {
            if c.is_digit(10) {
                self.eat_char();
            } else {
                break;
            }
        }
        self.input[start..self.current_pos].parse::<i64>().unwrap_or(0)
    }
}

