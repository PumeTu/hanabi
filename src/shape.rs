use std::convert::From;

pub struct Shape(pub Vec<usize>);

impl From<&[usize]> for Shape {
    fn from(item: &[usize]) -> Self {
        Shape(item.to_vec())
    }
}

impl Shape {
    pub fn num_elem(&self) -> f32 {
        self.0.iter().product()
    }
}
