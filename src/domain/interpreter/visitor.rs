use crate::domain::parser::expressions::*;
use crate::domain::parser::statements::*;


pub trait ExprVisitor {
    type Result;

    fn visit_literal(&mut self, expr: &mut Literal) -> Self::Result;
}
pub trait StmtVisitor {
    type Result;

    fn visit_block(&mut self, stmt: &mut Vec<Stmt>) -> Self::Result;
    fn visit_expression(&mut self, stmt: &mut Expr) -> Self::Result;
    fn visit_repeat(&mut self, repeat_stmt: &mut RepeatStmt) -> Self::Result;
    fn visit_set_color(&mut self, stmt: &mut SetColorStmt) -> Self::Result;
    fn visit_move_forward(&mut self, stmt: &mut MoveForwardStmt) -> Self::Result;
    fn visit_move_backward(&mut self, stmt: &mut MoveBackwardStmt) -> Self::Result;
    fn visit_rotate_right(&mut self, stmt: &mut RotateRightStmt) -> Self::Result;
    fn visit_rotate_left(&mut self, stmt: &mut RotateLeftStmt) -> Self::Result;
}

pub trait ExprVisitable {
    fn accept<V: ExprVisitor>(&mut self, visitor: &mut V) -> V::Result;
}

pub trait StmtVisitable {
    fn accept<V: StmtVisitor>(&mut self, visitor: &mut V) -> V::Result;
}



