pub enum Ops {
    UnaryOps,
    BinaryOps,
    LoadOps,
}

pub enum UnaryOps {
    LOG2,
    EXP2,
}

pub enum BinaryOps {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub enum ReduceOps {
    SUM,
    MAX,
}

pub enum LoadOps {
    EMPTY,
    CONST,
}
