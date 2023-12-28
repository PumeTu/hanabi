use crate::layout::Layout;
use crate::lazy::LazyBuffer;
use crate::ops::Ops;
use crate::shape::Shape;

pub struct Tensor<T> {
    buffer: LazyBuffer<T>,
    layout: Layout,
    requires_grad: Option<bool>,
}

impl<T> Tensor<T> {
    pub fn init(buffer: LazyBuffer<T>, shape: Shape) -> Self {
        let layout = Layout::init(&shape);
        Tensor {
            buffer,
            layout,
            requires_grad: None,
        }
    }

    pub fn zeros(shape: Shape) -> Self {}

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        unimplemented!();
    }
}
