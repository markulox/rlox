use std::{any::Any, collections::VecDeque};

mod token;
use token::{Token, TokenType};

use crate::err::{self, report::error, scan::ScanErr, ErrReport};
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

    pub fn scan_tokens(&mut self) -> Result<&VecDeque<Token>, ScanErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c_opt = self.source.chars().nth(self.current);
        match c_opt {
            Some(c) => {
                self.current = self.current + 1;
                c // This is character from index = self.current
            }
            None => {
                error(self.line, format!("No more character!").as_str());
                '\x04'
            }
        }
    }

    fn scan_token(&mut self) -> Result<(), ScanErr> {
        let c = self.advance();
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen, None)),
            ')' => Ok(self.add_token(TokenType::RightParen, None)),
            '{' => Ok(self.add_token(TokenType::LeftBrace, None)),
            '}' => Ok(self.add_token(TokenType::RightBrace, None)),
            ',' => Ok(self.add_token(TokenType::Comma, None)),
            '.' => Ok(self.add_token(TokenType::Dot, None)),
            '-' => Ok(self.add_token(TokenType::Minus, None)),
            '+' => Ok(self.add_token(TokenType::Plus, None)),
            ';' => Ok(self.add_token(TokenType::Semicolon, None)),
            '*' => Ok(self.add_token(TokenType::Star, None)),
            // 2 char cases
            '!' => {
                let t = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Ok(self.add_token(t, None))
            }
            '=' => {
                let t = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Ok(self.add_token(t, None))
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Ok(self.add_token(t, None))
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Ok(self.add_token(t, None))
            }
            _ => Err(ScanErr::UnknownChar(self.line, c)),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let lexeme = String::from(self.source[self.start..self.current].as_mut());
        self.tokens
            .push_back(Token::new(token_type, lexeme, literal, self.line));
    }

    fn match_char(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(c) = self.source.chars().nth(self.current) {
            if c != ch {
                return false;
            }
            self.current = self.current + 1;
            return true;
        } else {
            return false;
        }
    }
}
