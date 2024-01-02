use crate::{dtype::Dtype, layout::Layout, lazy::LazyBuffer, ops::Ops, shape::Shape};

pub struct Tensor {
    buffer: LazyBuffer,
    layout: Layout,
    ops: Ops,
    dtype: Dtype,
    requires_grad: Option<bool>,
}

impl Tensor {
    fn new(
        buffer: LazyBuffer,
        shape: &Shape,
        ops: Ops,
        dtype: Dtype,
        requires_grad: Option<bool>,
    ) -> Self {
        let layout = Layout::init(&shape);
        Tensor {
            buffer,
            layout,
            ops,
            dtype,
            requires_grad,
        }
    }

    pub fn zeros(shape: &Shape, dtype: Option<&str>, requires_grad: Option<bool>) -> Self {
        let dtype = Dtype::from_str(dtype);
        let buffer = LazyBuffer::init_const(0, &dtype, &shape);
        Self::init(buffer, &shape, Ops::LoadOps, dtype, requires_grad)
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }
}
