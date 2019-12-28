use std::fmt::{Display, Error, Formatter};

pub use infix::infix_op;
pub use prefix::prefix_op;

mod infix;
mod prefix;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    NotEqual,
}

#[derive(Debug)]
pub enum Lit {
    Str(String),
    Int(i64),
    FLoat(f64),
    Boolean(bool),
}

impl Display for Lit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Lit::Str(s) => write!(f, "\"{}\"", s),
            Lit::Int(i) => write!(f, "{}i", i),
            Lit::FLoat(ff) => write!(f, "{}f", ff),
            Lit::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Literal(Lit, usize),
    Identifier(String, usize),
    Let(Box<Expr>, usize),
    Group(Box<Expr>, usize),
    Block(Vec<Expr>, usize),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        start: usize,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expr::Literal(exp, _) => write!(f, "{}", exp),
            Expr::Identifier(id, _) => write!(f, "`{}`", id),
            Expr::Let(exp, _) => write!(f, "let {}", exp),
            Expr::Group(exp, _) => write!(f, "({})", exp),
            Expr::Binary { left, op, right, .. } => write!(f, "[{} {:?} {}]", left, op, right),
            Expr::Block(exps, _) => {
                writeln!(f, "{{")?;
                for exp in exps {
                    writeln!(f, "  {}", exp)?;
                }
                write!(f, "}}")
            }
        }
    }
}
