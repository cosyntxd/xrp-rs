pub struct Screen {
    pub width: u32,
    pub height: u32,
}
pub struct Pos2d {
    pub x: f64,
    pub y: f64,
}

impl Pos2d {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

pub enum Shapes {
    Image,
}
