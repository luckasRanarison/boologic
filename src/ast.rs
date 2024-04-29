use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Impl(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Var(char),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Or(lhs, rhs) => write!(f, "({lhs} ∨ {rhs})"),
            Expr::And(lhs, rhs) => write!(f, "({lhs} ∧ {rhs})"),
            Expr::Impl(lhs, rhs) => write!(f, "({lhs} ⭢ {rhs})"),
            Expr::Eq(lhs, rhs) => write!(f, "({lhs} ⭤ {rhs})"),
            Expr::Not(op) => write!(f, "¬{op}"),
            Expr::Var(name) => write!(f, "{name}"),
        }
    }
}
