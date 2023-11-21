#[derive(PartialEq, Clone)]
pub struct Location2d {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Size2d {
    // Add validation, must be positive.
    pub width: f64,
    pub height: f64
}
