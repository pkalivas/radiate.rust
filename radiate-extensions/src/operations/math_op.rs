use crate::operations::op::Op;

pub enum MathOp<T> {
    Add(u8, Box<dyn Fn(&[T]) -> T>),
    Subtract(u8, Box<dyn Fn(&[T]) -> T>),
    Multiply(u8, Box<dyn Fn(&[T]) -> T>),
    Divide(u8, Box<dyn Fn(&[T]) -> T>),
    Sum(u8, Box<dyn Fn(&[T]) -> T>)
}


impl<T> Op<T> for MathOp<T> {
    fn name(&self) -> &str {
        match self {
            MathOp::Add(_, _) => "Add",
            MathOp::Subtract(_, _) => "Subtract",
            MathOp::Multiply(_, _) => "Multiply",
            MathOp::Divide(_, _) => "Divide",
            MathOp::Sum(_, _) => "Sum"
        }
    }

    fn arity(&self) -> u8 {
        match self {
            MathOp::Add(arity, _) => *arity,
            MathOp::Subtract(arity, _) => *arity,
            MathOp::Multiply(arity, _) => *arity,
            MathOp::Divide(arity, _) => *arity,
            MathOp::Sum(arity, _) => *arity
        }
    }

    fn apply(&self, inputs: &[T]) -> T {
        match self {
            MathOp::Add(_, op) => (op)(inputs),
            MathOp::Subtract(_, op) => (op)(inputs),
            MathOp::Multiply(_, op) => (op)(inputs),
            MathOp::Divide(_, op) => (op)(inputs),
            MathOp::Sum(_, op) => (op)(inputs)
        }
    }
}
