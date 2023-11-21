use std::borrow::Cow;
use crate::domain::geometry::Size2d;

#[derive(PartialEq, Clone)]
pub struct Turtle {
    pub x: f64,
    pub y: f64,
    // Angle in rad.
    pub phi: f64,

    pub color: Cow<'static, str>,
}

impl Turtle {
    pub fn new_for_canvas(canvas_size: &Size2d) -> Turtle {
        Turtle {
            x: canvas_size.width / 2.0,
            y: canvas_size.height / 2.0,
            phi: std::f64::consts::PI / 2.0,
            color: "#000000".into(),
        }
    }
}
