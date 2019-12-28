use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Plus,
    Minus,
    Mul,
    Div,
    Modulo,
    Comma,
    SemiColon,
    Colon,

    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
    NotEqual,
    Dot,

    Str(&'a str),
    Int(i64),
    Float(f64),
    Boolean(bool),
    Identifier(&'a str),

    Let,
    Function,
    Return,
    For,
    While,
    If,
    Else,
    Null,
    Print,
}

impl<'a> TokenType<'a> {
    pub fn keywords() -> HashMap<&'static str, Self> {
        use TokenType::*;
        vec![(Let, "let"),
             (Function, "fn"),
             (Return, "return"),
             (For, "for"),
             (While, "while"),
             (If, "if"),
             (Else, "else"),
             (Null, "null"),
             (Print, "print"),
             (Boolean(true), "true"),
             (Boolean(false), "false")]
            .into_iter()
            .map(|(a, b)| (b, a))
            .collect()
    }

    pub fn infix_preced(&self) -> i32 {
        use TokenType::*;

        match self {
            Equal => 1,
            EqualEqual | Less | LessEqual | Greater | GreaterEqual | NotEqual => 2,
            Plus | Minus => 3,
            Mul | Div | Modulo => 4,
            LeftParen => 5,
            //Str(_) | Int(_) | Float(_) | Boolean(_) | Identifier(_) => 6,
            _ => 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    src: &'a str,
    tt: TokenType<'a>,
    start: usize,
    lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(src: &'a str, start: usize, end: usize, tt: TokenType<'a>) -> Self {
        Token { src, tt, start, lexeme: &src[start..=end] }
    }

    pub fn src(&self) -> &str { self.src }

    pub fn start(&self) -> usize { self.start }

    pub fn token_type(&self) -> TokenType { self.tt }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[{}, {:?}]", self.lexeme, self.tt)
    }
}