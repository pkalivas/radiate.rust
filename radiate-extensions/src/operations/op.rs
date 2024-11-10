use std::{ops::{Add, Div, Mul, Sub}, sync::Arc};

use rand::{prelude::Distribution, random};

use super::ops::Ops;

pub fn add<T>() -> Ops<T>
where
    T: Add<Output = T> + Clone
{
    Ops::Math("+", 2, Arc::new(|inputs: &[T]| inputs[0].clone() + inputs[1].clone()))
}

pub fn sub<T>() -> Ops<T>
where
    T: Sub<Output = T> + Clone
{
    Ops::Math("-", 2, Arc::new(|inputs: &[T]| inputs[0].clone() - inputs[1].clone()))
}

pub fn mul<T>() -> Ops<T>
where
    T: Mul<Output = T> + Clone
{
    Ops::Math("*", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn div<T>() -> Ops<T>
where
    T: Div<Output = T> + Clone
{
    Ops::Math("/", 2, Arc::new(|inputs: &[T]| inputs[0].clone() / inputs[1].clone()))
}

pub fn sum<T>() -> Ops<T>
where
    T: Add<Output = T> + Clone + Default
{
    Ops::Math("sum", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc + x.clone())))
}

pub fn prod<T>() -> Ops<T>
where
    T: Mul<Output = T> + Clone + Default
{
    Ops::Math("prod", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc * x.clone())))
}

pub fn neg<T>() -> Ops<T>
where
    T: std::ops::Neg<Output = T> + Clone + Default
{
    Ops::Math("neg", 1, Arc::new(|inputs: &[T]| -inputs[0].clone()))
}

pub fn pow<T>() -> Ops<T>
where
    T: Mul<Output = T> + Clone
{
    Ops::Math("pow", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn weight<T>() -> Ops<T>
where
    rand::distributions::Standard: Distribution<T>,
    T: Sub<Output = T> + Mul<Output = T> + Copy + Default
{
    let supplier = || random::<T>() - random::<T>();
    let operation = |inputs: &[T], weight: &T| inputs[0] * *weight;
    Ops::MutableConst("w", 1, supplier(), Arc::new(supplier), Arc::new(operation))
}

pub fn var<T>(index: usize) -> Ops<T>
where
    T: Clone
{
    let var_name = format!("x{}", index);
    Ops::Var(var_name, index)
}