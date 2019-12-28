use crate::error::ParsingErr;
use crate::expr::{BinaryOp, Expr};
use crate::lex::{Token, TokenType};
use crate::parser::Parser;

type Er = ParsingErr;

pub fn infix_op<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, left: Expr, token: Token<'a>) -> Result<Expr, Er> {
    use TokenType::*;
    match token.token_type() {
        Equal => {
            if let Expr::Identifier(_, _) = &left {
                let exp = parse_binary(parser, left, token, BinaryOp::Equal)?;
                if !parser.has_next(SemiColon, false) { Err(Er::token(token, "Statement not terminated properly")) } else { Ok(exp) }
            } else {
                Err(Er::token(token, "Expression can only be assigned to a variable"))
            }
        }
        Plus => parse_binary(parser, left, token, BinaryOp::Add),
        Minus => parse_binary(parser, left, token, BinaryOp::Sub),
        Mul => parse_binary(parser, left, token, BinaryOp::Mul),
        Div => parse_binary(parser, left, token, BinaryOp::Div),
        Modulo => parse_binary(parser, left, token, BinaryOp::Modulo),
        EqualEqual => parse_binary(parser, left, token, BinaryOp::EqualEqual),
        Less => parse_binary(parser, left, token, BinaryOp::Less),
        LessEqual => parse_binary(parser, left, token, BinaryOp::LessEqual),
        Greater => parse_binary(parser, left, token, BinaryOp::Greater),
        GreaterEqual => parse_binary(parser, left, token, BinaryOp::GreaterEqual),
        NotEqual => parse_binary(parser, left, token, BinaryOp::NotEqual),
        _ => Err(ParsingErr::token(token, format!("Unexpected infix token type `{:?}`", token.token_type())))
    }
}

fn parse_binary<'a, I: Iterator<Item=Token<'a>>>(parser: &mut Parser<'a, I>, left: Expr, token: Token, op: BinaryOp) -> Result<Expr, Er> {
    let err_fn = || Er::token(token, "Failed to parse right expression");
    let preced = token.token_type().infix_preced();
    let right = parser.parse_with_prec(preced)?.ok_or_else(err_fn)?;
    Ok(Expr::Binary { left: Box::new(left), op, right: Box::new(right), start: token.start() })
}