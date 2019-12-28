use std::iter::Peekable;

use crate::error::ParsingErr;
use crate::expr::{Expr, infix_op, prefix_op};
use crate::lex::{Token, TokenType};

pub struct Parser<'a, I: Iterator<Item=Token<'a>>> {
    tokens: Peekable<I>
}

impl<'a, I: Iterator<Item=Token<'a>>> Parser<'a, I> {
    pub fn new(itr: I) -> Self {
        Parser { tokens: itr.peekable() }
    }

    pub fn parse(&mut self) -> Result<Option<Expr>, ParsingErr> {
        self.parse_with_prec(0)
    }

    pub fn parse_with_prec(&mut self, preced: i32) -> Result<Option<Expr>, ParsingErr> {
        if let Some(token) = self.tokens.next() {
            let mut left = prefix_op(self, token)?;

            while preced < self.next_preced() {
                let next = self.tokens.next().unwrap();
                // println!("\nPBS {} curr: {:?} next: {:?}", left, token, next);
                left = infix_op(self, left, next)?;
            }
            return Ok(Some(left));
        }
        Ok(None)
    }

    pub fn next_preced(&mut self) -> i32 {
        self.tokens.peek().map(|t| t.token_type().infix_preced()).unwrap_or(0)
    }

    pub fn has_next(&mut self, tt: TokenType, consume: bool) -> bool {
        if let Some(token) = self.tokens.peek() {
            if token.token_type() == tt {
                if consume { self.tokens.next(); }
                return true;
            }
        }
        false
    }
}