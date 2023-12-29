use crate::layout::Layout;
use crate::lazy::LazyBuffer;
use crate::ops::Ops;
use crate::shape::Shape;

pub struct Tensor<T> {
    buffer: LazyBuffer<T>,
    layout: Layout,
    ops: Ops,
    requires_grad: Option<bool>,
}

impl<T> Tensor<T> {
    fn init(buffer: LazyBuffer<T>, shape: Shape, ops: Ops) -> Self {
        let layout = Layout::init(&shape);
        Tensor {
            buffer,
            layout,
            ops,
            requires_grad: None,
        }
    }

    pub fn zeros(&shape: Shape) -> Self {
        let buffer = LazyBuffer::new(vec![0.; shape.0.iter().product()]);
        Self::init(buffer, shape, Ops::LoadOps::CONST)
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }
}
