use core::f32;

use crate::{dtype::Dtype, shape::Shape};

pub struct LazyBuffer<T> {
    buffer: Vec<T>,
}

impl<T> LazyBuffer<T> {
    pub fn init_const(value: T, dtype: Dtype, shape: Shape) -> Self {
        let value = match dtype {
            Dtype::F32 => value as f32,
            Dtype::F64 => value as f64,
        };
        let data = vec![value; shape.num_elem()];
    }
}
