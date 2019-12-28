use crate::lex::Lexer;
use crate::parser::Parser;

pub mod lex;
pub mod error;
pub mod parser;
pub mod expr;

pub struct RLox {
    _inner: i32
}

impl RLox {
    pub fn default() -> Self {
        RLox { _inner: 0 }
    }

    pub fn exec<S: AsRef<str>>(&mut self, src: S) {
        let tokens = match Lexer::new(src.as_ref()).tokenize() {
            Err(e) => {
                eprintln!("{}", e);
                Vec::new()
            }
            Ok(tokens) => tokens
        };
        let mut parser = Parser::new(tokens.into_iter());
        match parser.parse() {
            Ok(Some(exp)) => println!("{}", exp),
            Ok(None) => println!("No expression"),
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}