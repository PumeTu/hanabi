pub trait Ops {
    fn forward(&self) -> Self;
    fn backward(&self) -> Self;
}

pub enum UnaryOps {
    LOG,
    EXP,
}

pub enum BinaryOps {
    ADD,
    SUB,
    MUL,
    DIV,
}
