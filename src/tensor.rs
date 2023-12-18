use crate::layout::Layout;
use crate::shape::Shape;

pub struct Tensor {
    layout: Layout,
}

impl Tensor {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }
}
