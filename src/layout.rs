use crate::shape::Shape;

pub struct Layout {
    pub shape: Shape,
    stride: Vec<usize>,
}

impl Layout {
    pub fn init(shape: &Shape) -> Self {
        let stride = Self::init_stride(&shape);
        Layout { shape, stride }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    fn init_stride(shape: &Shape) -> Vec<usize> {
        shape.0.iter().enumerate()
    }
}
