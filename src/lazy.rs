pub struct LazyBuffer<T> {
    buffer: Vec<T>,
}

impl<T> LazyBuffer<T> {
    pub fn new(buffer: Vec<T>) -> Self {
        LazyBuffer { buffer }
    }
}
