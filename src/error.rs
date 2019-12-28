use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Write;

use crate::lex::Token;

#[derive(Debug)]
pub struct ParsingErr {
    errors: Vec<String>
}

impl ParsingErr {
    pub fn token<S: AsRef<str>>(token: Token, msg: S) -> Self {
        ParsingErr::new(token.src(), &[(token.start(), msg.as_ref())])
    }

    pub fn new(src: &str, err: &[(usize, &str)]) -> Self {
        let mut errors = Vec::with_capacity(err.len());
        for &(index, msg) in err {
            let start = src[..index].rfind('\n').map(|p| p + 1).unwrap_or(0);
            let end = src[index..].find('\n').map(|p| index + p).unwrap_or(src.len());
            let mut error = String::new();
            writeln!(error, "{}", &src[start..end]).unwrap();
            for _ in start..index {
                write!(error, " ").unwrap();
            }
            write!(error, "^ {}, in line {}", msg, src[..index].split('\n').count()).unwrap();
            errors.push(error);
        }
        ParsingErr { errors }
    }
}

impl Display for ParsingErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for err in &self.errors {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl Error for ParsingErr {}
