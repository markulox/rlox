use std::{any::Any, collections::VecDeque, process::id};

mod keywords;

mod token;
use keywords::KEYWORDS;
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

    pub fn scan_tokens(&mut self) -> Result<&VecDeque<Token>, Vec<ScanErr>> {
        let mut err_vec = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                err_vec.push(e);
            }
        }
        if err_vec.len() > 0 {
            Err(err_vec)
        } else {
            Ok(&self.tokens)
        }
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
            '/' => {
                // Add support for multiline comment
                match self.peek() {
                    '/' => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            _ = self.advance();
                        }
                        return Ok(());
                    },
                    '*' => {
                        self.multiline_comment();
                        Ok(())
                    },
                    _ => {
                        Ok(self.add_token(TokenType::Slash, None))
                    }
                }
            }
            '"' => {
                self.string()
            }
            ' ' | '\r' | '\t' => Ok(()), // Consuming a meaningless char: Just do nothing here
            '\n' => {
                self.line = self.line + 1;
                Ok(())
            }
            '0' ..= '9' => { // Numeric case
                self.number()
            }
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                self.identifier();
                Ok(())
            }
            _ => {
                return Err(ScanErr::UnknownChar(self.line, c));
            },
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

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current+1).unwrap_or('\0')
    }

    fn string(&mut self) -> Result<(), ScanErr> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanErr::UnterminatedString(self.line));
        }
        self.advance();
        // self.start+1 and self.current-1 to trim out the '"'
        let str_val = String::from(self.source[self.start+1..self.current-1].as_mut());
        self.add_token(TokenType::String, Some(Box::new(str_val)));
        // Consume the '"' character (end of string)
        Ok(())
    }

    fn number(&mut self) -> Result<(), ScanErr> {
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric(){
                self.advance();
            }
        }
        let str_val = String::from(self.source[self.start..self.current].as_mut());
        let num_val = str_val.parse::<f64>()
            .map_err(|_| ScanErr::InvalidNumber(self.line, str_val))?;
        self.add_token(TokenType::Number, Some(Box::new(num_val)));
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let idntf = String::from(self.source[self.start..self.current].as_mut());
        let token_type: TokenType = if let Some(tt) = KEYWORDS.get(idntf.as_str()) {
            tt.clone()
        } else {
            TokenType::Identifier
        };
        self.add_token(token_type, None);
    }

    fn multiline_comment(&mut self) {
        let mut comment_count: usize = 1;
        self.advance(); // Consume the *
        while comment_count != 0 && !self.is_at_end() {
            let c = self.peek();
            println!("char='{c}'");
            match c {
                '/' => { // Start of nested comment
                    if self.peek_next() == '*' {
                        self.advance();
                        comment_count += 1;
                    }
                    self.advance();
                }, 
                '*' => {
                    if self.peek_next() == '/' {
                        self.advance();
                        comment_count -= 1;
                    }
                    self.advance();
                }, 
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => { self.advance(); }
            }
        }
    }
}
