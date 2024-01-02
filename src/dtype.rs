pub enum Dtype {
    F32,
    F64,
}

impl Dtype {
    pub fn from_str(dtype: Option<&str>) -> Dtype {
        match dtype {
            Some("f32") => Dtype::F32,
            Some("f64") => Dtype::F64,
            None => Dtype::F32, // Default precision
        }
    }
}
