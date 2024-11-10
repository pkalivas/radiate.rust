use std::ops::{Add, Div, Mul, Sub};

use rand::{prelude::Distribution, random};

use super::operation::Operation;

pub trait Op<T>  {
    fn name(&self) -> &str;
    fn arity(&self) -> u8;
    fn apply(&self, inputs: &[T]) -> T;
}

impl<T> std::fmt::Display for dyn Op<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn add<T>() -> impl Op<T> 
where
    T: Add<Output = T> + Clone
{
    Operation::Math("+", 2, Box::new(|inputs: &[T]| inputs[0].clone() + inputs[1].clone()))
}

pub fn sub<T>() -> impl Op<T> 
where
    T: Sub<Output = T> + Clone
{
    Operation::Math("-", 2, Box::new(|inputs: &[T]| inputs[0].clone() - inputs[1].clone()))
}

pub fn mul<T>() -> impl Op<T> 
where
    T: Mul<Output = T> + Clone
{
    Operation::Math("*", 2, Box::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn div<T>() -> impl Op<T> 
where
    T: Div<Output = T> + Clone
{
    Operation::Math("/", 2, Box::new(|inputs: &[T]| inputs[0].clone() / inputs[1].clone()))
}

pub fn sum<T>() -> impl Op<T> 
where
    T: Add<Output = T> + Clone + Default
{
    Operation::Math("sum", 2, Box::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc + x.clone())))
}

pub fn prod<T>() -> impl Op<T> 
where
    T: Mul<Output = T> + Clone + Default
{
    Operation::Math("prod", 2, Box::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc * x.clone())))
}

pub fn weight<T>() -> impl Op<T> 
where
    rand::distributions::Standard: Distribution<T>,
    T: Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    let supplier = || random::<T>() - random::<T>();
    let operation = |inputs: &[T], weight: &T| inputs[0] * *weight;
    Operation::MutableConst("w", 1, supplier(), Box::new(supplier), Box::new(operation))
}

pub fn var<T>(index: usize) -> impl Op<T> 
where
    T: Clone
{
    let var_name = format!("x{}", index);
    Operation::Var(var_name, index)
}