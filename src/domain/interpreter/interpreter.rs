use std::borrow::Cow;

use wasm_bindgen::{JsCast, JsValue};

use crate::domain::config::CANVAS_ID;
use crate::domain::errors::InterpreterError;
use crate::domain::interpreter::visitor::{ExprVisitable, ExprVisitor, StmtVisitable, StmtVisitor};
use crate::domain::parser::expressions::{Expr, Literal};
use crate::domain::parser::statements::{MoveBackwardStmt, MoveForwardStmt, RepeatStmt, RotateLeftStmt, RotateRightStmt, SetColorStmt, Stmt};
use crate::domain::turtle::Turtle;

#[derive(Debug, PartialEq, Clone)]
pub enum Obj {
    /// Represents types that can be used for return value.
    /// They are can be a result of an expression.
    Num(f64),
    Color(String),
}

pub struct Interpreter {
    pub return_value: Option<Obj>,
    turtle: Turtle,
}

impl Interpreter {
    pub fn new(
        turtle: Turtle
    ) -> Self {
        Interpreter {
            turtle,
            return_value: None,
        }
    }

    pub fn interpret_statements(&mut self, statements: &mut Vec<Stmt>) -> Result<(), InterpreterError> {
        for statement in statements {
            if self.return_value != None {
                return Ok(());
            }
            statement.accept(self)?;
        }
        Ok(())
    }

    fn clear_canvas(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        // TODO: Refactor canvas width and height.
        context.clear_rect(0.0, 0.0, 365.0, 365.0);
    }
    fn render_turtle(&mut self){
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
        let path = web_sys::Path2d::new_with_path_string("M42.706,63.979C41.467,64.827 39.973,64.849 39.22,63.974C38.393,63.011 38.773,61.324 40.07,60.209C40.177,60.117 40.286,60.032 40.398,59.954C38.743,55.63 38.934,51.169 40.952,46.921C40.46,46.548 40.022,46.002 39.723,45.331C39,43.712 39.355,41.978 40.515,41.46C41.409,41.061 42.522,41.501 43.316,42.465C44.181,40.961 45.28,39.376 46,39.54C45.537,38.56 45.285,38.494 45.26,37.41C45.262,34.996 46.368,33.71 48.26,32.21C49.26,31.412 50.69,31.412 51.69,32.21C53.573,33.717 54.676,34.998 54.69,37.41C54.693,38.487 54.472,38.553 54.04,39.54C54.745,39.379 55.804,40.898 56.65,42.374C57.646,41.38 58.992,41.066 59.882,41.699C60.917,42.435 60.944,44.176 59.943,45.583C59.643,46.005 59.284,46.35 58.9,46.606C61.048,50.889 61.326,55.404 59.712,59.791C60.044,60.047 60.348,60.376 60.601,60.767C61.519,62.19 61.398,63.904 60.331,64.593C59.472,65.147 58.26,64.869 57.341,63.998C56.498,64.994 55.264,66.263 54.21,66.59C53.138,66.922 51.435,67.563 49.82,67.553C48.301,67.544 46.869,66.903 45.86,66.59C44.798,66.261 43.552,64.979 42.706,63.979Z".into()).unwrap();

        let canvas: web_sys::HtmlCanvasElement =
            canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.save();

        // Unfortunately, the Path2d struct in web_sys does not provide a direct way to find its dimensions.
        // This is because a Path2d object represents a path made up of points, arcs, or curves,
        // and it does not have a specific width or height like a Rect or Image object.
        // Values 50.0 are result of experimentation.
        let _ = context.translate(self.turtle.x - 50.0, self.turtle.y - 50.0);
        // Applying rotation and scaling is trickier.
        // Just translating to correct location is good enough.
        context.fill_with_path_2d(&path);
        context.restore();
    }

    /// Function should be invoked only once per script.
    pub fn interpret_script(
        &mut self,
        block: &mut Vec<Stmt>,
    ) -> Result<(), InterpreterError> {
        self.clear_canvas();
        self.interpret_block(block)?;
        self.render_turtle();
        Ok(())
    }

    fn interpret_block(
        &mut self,
        statements: &mut Vec<Stmt>,
    ) -> Result<(), InterpreterError> {
        self.interpret_statements(statements)
    }

    fn evaluate(&mut self, expr: &mut Expr) -> Result<Obj, InterpreterError> {
        expr.accept(self)
    }
}

impl StmtVisitor for Interpreter {
    type Result = Result<(), InterpreterError>;

    fn visit_block(&mut self, stmts: &mut Vec<Stmt>) -> Self::Result {
        self.interpret_block(stmts)
    }

    fn visit_expression(&mut self, expr: &mut Expr) -> Self::Result {
        self.evaluate(expr).map(|_| ())
    }

    fn visit_repeat(&mut self, repeat_stmt: &mut RepeatStmt) -> Self::Result {
        for _ in 0..repeat_stmt.count {
            if self.return_value != None {
                break;
            }
            repeat_stmt.body.accept(self)?;
        }
        Ok(())
    }

    fn visit_set_color(&mut self, stmt: &mut SetColorStmt) -> Self::Result {
        self.turtle.color = Cow::Owned(stmt.color.to_string());
        Ok(())
    }

    fn visit_move_forward(&mut self, stmt: &mut MoveForwardStmt) -> Self::Result {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap();

        let canvas: web_sys::HtmlCanvasElement =
            canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();
        context.move_to(self.turtle.x, self.turtle.y);
        self.turtle.x = self.turtle.x + stmt.distance * self.turtle.phi.cos();
        self.turtle.y = self.turtle.y - stmt.distance * self.turtle.phi.sin();
        context.line_to(self.turtle.x, self.turtle.y);
        context.set_stroke_style(
            &JsValue::from_str(self.turtle.color.as_ref())
        );
        context.stroke();
        Ok(())
    }

    fn visit_move_backward(&mut self, stmt: &mut MoveBackwardStmt) -> Self::Result {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap();

        let canvas: web_sys::HtmlCanvasElement =
            canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();
        context.move_to(self.turtle.x, self.turtle.y);
        self.turtle.x = self.turtle.x - stmt.distance * self.turtle.phi.cos();
        self.turtle.y = self.turtle.y + stmt.distance * self.turtle.phi.sin();
        context.line_to(self.turtle.x, self.turtle.y);
        context.set_stroke_style(
            &JsValue::from_str(self.turtle.color.as_ref())
        );
        context.stroke();
        Ok(())
    }

    fn visit_rotate_right(&mut self, stmt: &mut RotateRightStmt) -> Self::Result {
        self.turtle.phi = self.turtle.phi - stmt.angular_distance * std::f64::consts::PI/180.0;
        Ok(())
    }

    fn visit_rotate_left(&mut self, stmt: &mut RotateLeftStmt) -> Self::Result {
        self.turtle.phi = self.turtle.phi + stmt.angular_distance * std::f64::consts::PI/180.0;
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Result = Result<Obj, InterpreterError>;

    fn visit_literal(&mut self, literal: &mut Literal) -> Self::Result {
        match literal {
            Literal::Number(number) => Ok(Obj::Num(*number)),
            Literal::Color(string) => Ok(Obj::Color(string.to_string())),
        }
    }
}
