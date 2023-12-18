use crate::shape::Shape;

pub struct Layout {
    pub shape: Shape,
    stride: Vec<usize>,
}

impl Layout {
    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}
