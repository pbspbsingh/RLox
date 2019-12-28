use crate::error::ParsingErr;
use crate::expr::{BinaryOp, Expr, Lit};
use crate::lex::{Token, TokenType};
use crate::parser::Parser;

type Er = ParsingErr;

pub fn prefix_op<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, token: Token<'a>) -> Result<Expr, Er> {
    use TokenType::*;
    match token.token_type() {
        Str(s) => Ok(Expr::Literal(Lit::Str(s.to_owned()), token.start())),
        Int(v) => Ok(Expr::Literal(Lit::Int(v), token.start())),
        Float(v) => Ok(Expr::Literal(Lit::FLoat(v), token.start())),
        Boolean(v) => Ok(Expr::Literal(Lit::Boolean(v), token.start())),
        Identifier(id) => Ok(Expr::Identifier(id.to_owned(), token.start())),
        Let => parse_let(parser, token),
        LeftParen => parse_group(parser, token),
        LeftBrace => parse_block(parser, token),
        _ => Err(Er::token(token, format!("Unexpected prefix token type `{:?}`", token.token_type())))
    }
}

fn parse_let<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, token: Token) -> Result<Expr, ParsingErr> {
    let exp = parser.parse()?.ok_or_else(|| ParsingErr::token(token, "Couldn't parse variable name of let"))?;
    if let Expr::Binary { op, .. } = &exp {
        if *op == BinaryOp::Equal {
            return Ok(Expr::Let(Box::new(exp), token.start()));
        } else {
            return Err(Er::token(token, "expression should be assigned with '=' operator"));
        }
    }
    Err(Er::token(token, "Failed to parse let expression"))
}

fn parse_group<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, token: Token) -> Result<Expr, Er> {
    let exp = parser.parse()?.ok_or_else(|| ParsingErr::token(token, "Couldn't parse nested expression"))?;
    if !parser.has_next(TokenType::RightParen, true) {
        return Err(Er::token(token, "No closing parenthesis found"));
    }
    Ok(Expr::Group(Box::new(exp), token.start()))
}

fn parse_block<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, token: Token) -> Result<Expr, Er> {
    let mut exprs = Vec::new();
    while let Some(expr) = parser.parse()? {
        exprs.push(expr);

        if parser.has_next(TokenType::RightBrace, true) {
            break;
        }
        if !parser.has_next(TokenType::SemiColon, true) {
            return Err(Er::token(token, "Statement didn't close with a ';'"));
        }
    }

    Ok(Expr::Block(exprs, token.start()))
}