use std::{ops::{Add, Div, Mul, Sub, Neg}, sync::Arc};

use rand::{prelude::Distribution, distributions::Standard, random};

use super::ops::Ops;

pub fn add<T: Add<Output = T> + Clone>() -> Ops<T> {
    Ops::Math("+", 2, Arc::new(|inputs: &[T]| inputs[0].clone() + inputs[1].clone()))
}

pub fn sub<T: Sub<Output = T> + Clone>() -> Ops<T> {
    Ops::Math("-", 2, Arc::new(|inputs: &[T]| inputs[0].clone() - inputs[1].clone()))
}

pub fn mul<T: Mul<Output = T> + Clone>() -> Ops<T> {
    Ops::Math("*", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn div<T: Div<Output = T> + Clone>() -> Ops<T> {
    Ops::Math("/", 2, Arc::new(|inputs: &[T]| inputs[0].clone() / inputs[1].clone()))
}

pub fn sum<T: Add<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Math("sum", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc + x.clone())))
}

pub fn prod<T: Mul<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Math("prod", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc * x.clone())))
}

pub fn neg<T: Neg<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Math("neg", 1, Arc::new(|inputs: &[T]| -inputs[0].clone()))
}

pub fn pow<T: Mul<Output = T> + Clone>() -> Ops<T> {
    Ops::Math("pow", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn weight<T: Sub<Output = T> + Mul<Output = T> + Copy + Default>() -> Ops<T>
where
    Standard: Distribution<T>,
{
    let supplier = || random::<T>() - random::<T>();
    let operation = |inputs: &[T], weight: &T| inputs[0] * *weight;
    Ops::MutableConst("w", 1, supplier(), Arc::new(supplier), Arc::new(operation))
}

pub fn var<T: Clone>(index: usize) -> Ops<T> {
    let var_name = format!("x{}", index);
    Ops::Var(var_name, index)
}