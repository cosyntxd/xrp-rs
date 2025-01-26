/// 
pub struct Pos2 {
    pub x: f64,
    pub y: f64,
}

impl Pos2 {
    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

pub enum Shapes {
    Image
}