use std::{ops::{Add, Div, Mul, Sub, Neg}, sync::Arc};

use rand::{prelude::Distribution, distributions::Standard, random};

pub enum Ops<T> 
where
    T: Clone
{
    Fn(&'static str, u8, Arc<dyn Fn(&[T]) -> T>),
    Var(String, usize),
    Value(T),
    Const(&'static str, T),
    MutableConst(&'static str, u8, T, Arc<dyn Fn() -> T>, Arc<dyn Fn(&[T], &T) -> T>),
}

impl<T> Ops<T>
where
    T: Clone
{
    pub fn name(&self) -> &str {
        match self {
            Ops::Fn(name, _, _) => name,
            Ops::Var(name, _) => name,
            Ops::Value(_) => "value",
            Ops::Const(name, _) => name,
            Ops::MutableConst(name, _, _, _, _) => name,
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Ops::Fn(_, arity, _) => *arity,
            Ops::Var(_, _) => 0,
            Ops::Value(_) => 0,
            Ops::Const(_, _) => 0,
            Ops::MutableConst(_, arity, _, _, _) => *arity,
        }
    }

    pub fn apply(&self, inputs: &[T]) -> T {
        match self {
            Ops::Fn(_, _, op) => op(inputs),
            Ops::Var(_, index) => inputs[*index].clone(),
            Ops::Value(value) => value.clone(),
            Ops::Const(_, value) => value.clone(),
            Ops::MutableConst(_, _, value, _, operation) => operation(inputs, value),
        }
    }

    pub fn new_instance(&self) -> Ops<T> {
        match self {
            Ops::Fn(name, arity, op) => Ops::Fn(name, *arity, op.clone()),
            Ops::Var(name, index) => Ops::Var(name.clone(), *index),
            Ops::Value(value) => Ops::Value(value.clone()),
            Ops::Const(name, value) => Ops::Const(name, value.clone()),
            Ops::MutableConst(name, arity, _, get_value, operation) => Ops::MutableConst(name, *arity, get_value(), get_value.clone(), operation.clone()),
        }
    }
}

impl<T> Clone for Ops<T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        match self {
            Ops::Fn(name, arity, op) => Ops::Fn(name, *arity, op.clone()),
            Ops::Var(name, index) => Ops::Var(name.clone(), *index),
            Ops::Value(value) => Ops::Value(value.clone()),
            Ops::Const(name, value) => Ops::Const(name, value.clone()),
            Ops::MutableConst(name, arity, value, get_value, operation) => Ops::MutableConst(name, *arity, value.clone(), get_value.clone(), operation.clone()),
        }
    }
}

impl<T> PartialEq for Ops<T>
where
    T: Clone
{
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<T> std::fmt::Display for Ops<T>
where
    T: Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<T> Default for Ops<T>
where
    T: Clone + Default
{
    fn default() -> Self {
        Ops::Const("default", T::default())
    }
}

impl<T> std::fmt::Debug for Ops<T>
where
    T: Clone + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ops::Fn(name, arity, _) => write!(f, "Fn: {}({})", name, arity),
            Ops::Var(name, index) => write!(f, "Var: {}({})", name, index),
            Ops::Value(value) => write!(f, "Value: {:?}", value),
            Ops::Const(name, value) => write!(f, "Const: {}({:?})", name, value),
            Ops::MutableConst(name, arity, value, _, _) => write!(f, "MutConst: {}({})({:.2?})", name, arity, value),
        }
    }
}



pub fn add<T: Add<Output = T> + Clone>() -> Ops<T> {
    Ops::Fn("+", 2, Arc::new(|inputs: &[T]| inputs[0].clone() + inputs[1].clone()))
}

pub fn sub<T: Sub<Output = T> + Clone>() -> Ops<T> {
    Ops::Fn("-", 2, Arc::new(|inputs: &[T]| inputs[0].clone() - inputs[1].clone()))
}

pub fn mul<T: Mul<Output = T> + Clone>() -> Ops<T> {
    Ops::Fn("*", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
}

pub fn div<T: Div<Output = T> + Clone>() -> Ops<T> {
    Ops::Fn("/", 2, Arc::new(|inputs: &[T]| inputs[0].clone() / inputs[1].clone()))
}

pub fn sum<T: Add<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Fn("sum", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc + x.clone())))
}

pub fn prod<T: Mul<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Fn("prod", 2, Arc::new(|inputs: &[T]| inputs
        .iter()
        .fold(T::default(), |acc, x| acc * x.clone())))
}

pub fn neg<T: Neg<Output = T> + Clone + Default>() -> Ops<T> {
    Ops::Fn("neg", 1, Arc::new(|inputs: &[T]| -inputs[0].clone()))
}

pub fn pow<T: Mul<Output = T> + Clone>() -> Ops<T> {
    Ops::Fn("pow", 2, Arc::new(|inputs: &[T]| inputs[0].clone() * inputs[1].clone()))
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

pub fn sigmoid() -> Ops<f32> {
    Ops::Fn("sigmoid", 1, Arc::new(|inputs: &[f32]| {
        let sum = inputs.iter().fold(0_f32, |acc, x| acc + x);
        1_f32 / (1_f32 + (-sum).exp())
    }))
}

pub fn relu() -> Ops<f32> {
    Ops::Fn("relu", 1, Arc::new(|inputs: &[f32]| {
        let sum = inputs.iter().fold(0_f32, |acc, x| acc + x);
        if sum > 0_f32 {
            sum
        } else {
            0_f32
        }
    }))
}

pub fn tanh() -> Ops<f32> {
    Ops::Fn("tanh", 1, Arc::new(|inputs: &[f32]| {
        inputs
            .iter()
            .fold(0_f32, |acc, x| acc + x)
            .tanh()
    }))
}

pub fn linear() -> Ops<f32> {
    Ops::Fn("linear", 1, Arc::new(|inputs: &[f32]| {
        inputs
            .iter()
            .fold(0_f32, |acc, x| acc + x)
    }))
}
