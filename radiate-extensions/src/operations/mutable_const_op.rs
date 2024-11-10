use super::op::Op;


pub struct MutableConstOp<T> {
    pub name: &'static str,
    pub arity: u8,
    pub value: T,
    pub supplier: Box<dyn Fn() -> T>,
    pub operation: Box<dyn Fn(&[T], &T) -> T>,
}

impl<T> MutableConstOp<T> {
    pub fn new(
        name: &'static str,
        arity: u8,
        suppler: Box<dyn Fn() -> T>, 
        operation: Box<dyn Fn(&[T], &T) -> T>) -> MutableConstOp<T> 
    {
        MutableConstOp {
            name: name,
            arity: arity,
            value: suppler(),
            supplier: suppler,
            operation: operation,
        }
    }
}

impl<T> Op<T> for MutableConstOp<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn arity(&self) -> u8 {
        self.arity
    }

    fn apply(&self, inputs: &[T]) -> T {
        (self.operation)(inputs, &self.value)
    }
}