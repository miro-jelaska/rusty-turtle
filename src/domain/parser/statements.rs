use crate::domain::interpreter::visitor::{StmtVisitable, StmtVisitor};
use crate::domain::parser::expressions::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct RepeatStmt {
    pub count: usize,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetColorStmt {
    pub color: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MoveForwardStmt {
    pub distance: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MoveBackwardStmt {
    pub distance: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RotateRightStmt {
    pub angular_distance: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RotateLeftStmt {
    pub angular_distance: f64,
}

// Statements do not result in a value. They are just evaluated.
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Repeat(RepeatStmt),
    SetColor(SetColorStmt),
    MoveForward(MoveForwardStmt),
    MoveBack(MoveBackwardStmt),
    RotateRight(RotateRightStmt),
    RotateLeft(RotateLeftStmt),
}

impl StmtVisitable for Stmt {
    fn accept<V: StmtVisitor>(&mut self, visitor: &mut V) -> V::Result {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Block(stmts) => visitor.visit_block(stmts),
            Stmt::Repeat(stmt) => visitor.visit_repeat(stmt),
            Stmt::SetColor(stmt) => visitor.visit_set_color(stmt),
            Stmt::MoveForward(stmt) => visitor.visit_move_forward(stmt),
            Stmt::MoveBack(stmt) => visitor.visit_move_backward(stmt),
            Stmt::RotateRight(stmt) => visitor.visit_rotate_right(stmt),
            Stmt::RotateLeft(stmt) => visitor.visit_rotate_left(stmt),
        }
    }
}