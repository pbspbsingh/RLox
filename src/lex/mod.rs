use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

pub use token::Token;
pub use token::TokenType;

use crate::error::ParsingErr;

mod token;
#[cfg(test)]
mod test;

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<CharIndices<'a>>,
    tokens: Vec<Token<'a>>,
    errors: Vec<(usize, &'static str)>,
    keywords: HashMap<&'a str, TokenType<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let chars = src.char_indices().peekable();
        let keywords = TokenType::keywords();
        Lexer { src, chars, tokens: Vec::new(), errors: Vec::new(), keywords }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token<'a>>, ParsingErr> {
        use TokenType::*;

        while let Some((idx, curr)) = self.chars.next() {
            match curr {
                '#' => self.skip(true),
                '(' => self.add_char_token(idx, LeftParen),
                ')' => self.add_char_token(idx, RightParen),
                '{' => self.add_char_token(idx, LeftBrace),
                '}' => self.add_char_token(idx, RightBrace),
                '[' => self.add_char_token(idx, LeftSquare),
                ']' => self.add_char_token(idx, RightSquare),
                '+' => self.add_char_token(idx, Plus),
                '-' => self.add_char_token(idx, Minus),
                '*' => self.add_char_token(idx, Mul),
                '/' => self.add_char_token(idx, Div),
                '%' => self.add_char_token(idx, Modulo),
                ',' => self.add_char_token(idx, Comma),
                ';' => self.add_char_token(idx, SemiColon),
                ':' => self.add_char_token(idx, Colon),

                '=' => self.two_char_token(idx, '=', Equal, EqualEqual),
                '<' => self.two_char_token(idx, '=', Less, LessEqual),
                '>' => self.two_char_token(idx, '=', Greater, GreaterEqual),
                '!' => self.two_char_token(idx, '=', Not, NotEqual),

                '.' => self.add_float(idx, idx),
                '0'..='9' => self.add_int(idx),
                '\'' | '"' => self.add_str(idx, curr),
                'a'..='z' | 'A'..='Z' | '_' => self.add_identifier(idx),
                ' ' | '\t' | '\n' | '\r' => { /* Skip white spaces */ }
                _ => self.errors.push((idx, "Illegal character"))
            }
        }
        if self.errors.is_empty() {
            let mut tokens = vec![Token::new(self.src, 0, 0, TokenType::LeftBrace)];
            tokens.extend(self.tokens);
            tokens.push(Token::new(self.src, self.src.len() - 1, self.src.len() - 1, TokenType::RightBrace));
            Ok(tokens)
        } else {
            Err(ParsingErr::new(self.src, &self.errors))
        }
    }

    fn add_char_token(&mut self, idx: usize, tt: TokenType<'a>) {
        self.tokens.push(Token::new(self.src, idx, idx, tt));
    }

    fn two_char_token(&mut self, start: usize, expected: char, tt1: TokenType<'a>, tt2: TokenType<'a>) {
        if let Some(&(end, next)) = self.chars.peek() {
            if next == expected {
                let _ = self.chars.next();
                self.tokens.push(Token::new(self.src, start, end, tt2));
                return;
            }
        }
        self.tokens.push(Token::new(self.src, start, start, tt1));
    }

    fn add_float(&mut self, start: usize, mut end: usize) {
        while let Some(&(e, n)) = self.chars.peek() {
            match n {
                '0'..='9' => {
                    self.chars.next();
                    end = e;
                }
                '.' => {
                    self.errors.push((e, "'.' is not expected here"));
                    self.skip(false);
                    return;
                }
                _ => break,
            }
        }
        if start == end {
            self.add_char_token(start, TokenType::Dot);
        } else {
            let val = self.src[start..=end].parse().unwrap();
            self.tokens.push(Token::new(self.src, start, end, TokenType::Float(val)));
        }
    }

    fn add_int(&mut self, start: usize) {
        let mut end = start;
        while let Some(&(e, n)) = self.chars.peek() {
            match n {
                '0'..='9' => {
                    self.chars.next();
                    end = e;
                }
                '.' => {
                    self.chars.next();
                    self.add_float(start, e);
                    return;
                }
                _ => break,
            }
        }
        let int = self.src[start..=end].parse().unwrap();
        self.tokens.push(Token::new(self.src, start, end, TokenType::Int(int)));
    }

    fn skip(&mut self, skip_semi: bool) {
        while let Some((_, c)) = self.chars.next() {
            if c == '\n' || (!skip_semi && c == ';') { break; }
        }
    }

    fn add_str(&mut self, start: usize, enclosing: char) {
        let mut end = start;
        let mut prev = ' ';
        while let Some((e, n)) = self.chars.next() {
            if prev != '\\' && n == enclosing {
                end = e;
                break;
            }
            prev = n;
        }
        if start == end {
            self.errors.push((start, "String literal is not terminated"));
            self.skip(false);
        } else {
            self.tokens.push(Token::new(self.src, start, end, TokenType::Str(&self.src[start + 1..end])));
        }
    }

    fn add_identifier(&mut self, start: usize) {
        let mut end = start;
        while let Some(&(e, next)) = self.chars.peek() {
            end = match next {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.chars.next();
                    e
                }
                _ => break
            };
        }
        let ident = &self.src[start..=end];
        let tt = TokenType::Identifier(ident);
        let tt = self.keywords.get(ident).unwrap_or(&tt);
        self.tokens.push(Token::new(self.src, start, end, *tt));
    }
}