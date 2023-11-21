use crate::domain::interpreter::visitor::{ExprVisitable, ExprVisitor};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Color(String),
}

// Expressions result in a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
}

impl ExprVisitable for Expr {
    fn accept<V: ExprVisitor>(&mut self, visitor: &mut V) -> V::Result {
        match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}