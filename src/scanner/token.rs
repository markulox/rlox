use std::any::{self, Any};

pub enum TokenNano {
    LeftParen(usize, usize),
    RightParen(usize, usize),
    LeftBrace(usize, usize),
    RightBrace(usize, usize),
    Comma(usize, usize),
    Dot(usize, usize),
    Minus(usize, usize),
    Plus(usize, usize),
    Semicolon(usize, usize),
    Slash(usize, usize),
    Star(usize, usize),

    Bang(usize, usize),
    BangEqual(usize, usize),
    Equal(usize, usize),
    EqualEqual(usize, usize),
    Greater(usize, usize),
    GreaterEqual(usize, usize),
    Less(usize, usize),
    LessEqual(usize, usize),

    Identifier(usize, usize, String),
    String(usize, usize, String),
    Number(usize, usize, String),

    And(usize, usize),
    Class(usize, usize),
    Else(usize, usize),
    False(usize, usize),
    Fun(usize, usize),
    For(usize, usize),
    If(usize, usize),
    Nil(usize, usize),
    Or(usize, usize),
    Print(usize, usize),
    Return(usize, usize),
    Super(usize, usize),
    This(usize, usize),
    True(usize, usize),
    Var(usize, usize),
    While(usize, usize),

    EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub type OptionalAny = Option<Box<dyn Any>>;

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: OptionalAny,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: OptionalAny, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let tt = self.r#type;
//         let lexeme = self.lexeme;
//         let literal = self.literal;
//         let line = self.line;
//         write!(f, "{tt}", )
//     }
// }
