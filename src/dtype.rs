pub enum Dtype {
    F16,
    F32,
    F64,
}

impl Dtype {
    pub fn from_str(dtype: Option<&str>) -> Dtype {
        match dtype {
            Some("f16") => Dtype::F16,
            Some("f32") => Dtype::F32,
            Some("f64") => Dtype::F64,
            None => Dtype::F32, // Default precision
        }
    }
}
