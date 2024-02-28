from collections import DynamicVector
from tensor import Tensor, TensorSpec

@value
struct Node:
    """A node in the computation graph
    Args:
        tensor: the tensor stored
        requires_grad (Bool): whether the tensor requries a gradient to be calculated or not.
    """

    var tensor: Tensor[DType.float32]
    var requires_grad: Bool
    var childrens: DynamicVector[Node]

    fn __init__(inout self, tensor: Tensor[DType.float32], requires_grad: Bool):
        self.tensor = tensor
        self.requires_grad = requires_grad

    fn __copyinit__(inout self, existing: Self):
        self.tensor = existing.tensor
        self.requires_grad = existing.requires_grad

    fn __moveinit__(inout self, existing: Self):
        self.tensor = existing.tensor
        self.requires_grad = existing.requires_grad

    fn __add__(self: Self, other: Self) -> Self:
        ...
