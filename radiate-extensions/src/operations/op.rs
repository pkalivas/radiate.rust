use std::ops::{Add, Div, Mul, Sub};

use rand::{prelude::Distribution, random};

use crate::operations::math_op::MathOp;
use crate::operations::var_op::VarOp;
use crate::operations::mutable_const_op::MutableConstOp;

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
    T: Add<Output = T> + Copy
{
    MathOp::Add(2, Box::new(|inputs: &[T]| inputs[0] + inputs[1]))
}

pub fn sub<T>() -> impl Op<T> 
where
    T: Sub<Output = T> + Copy
{
    MathOp::Subtract(2, Box::new(|inputs: &[T]| inputs[0] - inputs[1]))
}

pub fn mul<T>() -> impl Op<T> 
where
    T: Mul<Output = T> + Copy
{
    MathOp::Multiply(2, Box::new(|inputs: &[T]| inputs[0] * inputs[1]))
}

pub fn div<T>() -> impl Op<T> 
where
    T: Div<Output = T> + Copy
{
    MathOp::Divide(2, Box::new(|inputs: &[T]| inputs[0] / inputs[1]))
}

pub fn sum<T>() -> impl Op<T> 
where
    T: Add<Output = T> + Copy + Default
{
    MathOp::Sum(2, Box::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, &x| acc + x)))
}

pub fn prod<T>() -> impl Op<T> 
where
    T: Mul<Output = T> + Copy + Default
{
    MathOp::Prod(2, Box::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, &x| acc * x)))
}

pub fn weight<T>() -> impl Op<T> 
where
    rand::distributions::Standard: Distribution<T>,
    T: Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    let supplier = || random::<T>() - random::<T>();
    let operation = |inputs: &[T], weight: &T| inputs[0] * *weight;
    MutableConstOp::new("w", 1, Box::new(supplier), Box::new(operation))
}

pub fn var<T>(index: usize) -> impl Op<T> 
where
    T: Clone
{
    let var_name = format!("x{}", index);
    VarOp::new(var_name, index)
}