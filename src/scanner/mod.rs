use std::collections::VecDeque;

mod token;
use token::{Token, TokenType};

use crate::err::{self, report::error, scan::ScanErr, Err};
pub struct Scanner {
    source: String,
    tokens: VecDeque<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(str_source: String) -> Self {
        Scanner {
            source: str_source,
            tokens: VecDeque::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<VecDeque<Token>, ScanErr> {
        let v = VecDeque::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        return Ok(v);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or_else(|| {
            error(self.line, format!("No more character!").as_str());
            '\x04'
        })
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => error(self.line, format!("Unexpected character: {c}").as_str()),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = String::from(self.source[self.start..self.current].as_mut());
        self.tokens
            .push_back(Token::new(token_type, lexeme, None, self.line));
    }
}
