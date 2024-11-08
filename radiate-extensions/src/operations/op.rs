use std::ops::Add;

use crate::operations::math_op::MathOp;

pub trait Op<T> {
    fn name(&self) -> &str;
    fn arity(&self) -> u8;
    fn apply(&self, inputs: &[T]) -> T;
}

impl<T> std::fmt::Display for dyn Op<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<T, F> Op<T> for F where F: Fn(&[T]) -> T {
    fn name(&self) -> &str {
        "Op"
    }

    fn arity(&self) -> u8 {
        0
    }

    fn apply(&self, inputs: &[T]) -> T {
        (self)(inputs)
    }
}


pub fn add<T>() -> impl Op<T> 
where
    T: Add<Output = T> + Clone
{
    MathOp::Add(2, Box::new(|inputs: &[T]| inputs[0].clone() + inputs[1].clone()))
}