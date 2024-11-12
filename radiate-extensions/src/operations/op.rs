use std::{ops::{Add, Div, Mul, Sub, Neg}, sync::Arc};

use rand::{prelude::Distribution, distributions::Standard, random};

use num_traits::{Float, NumCast};

const MAX_VALUE: f32 = 1e+6_f32;
const MIN_VALUE: f32 = -1e+6_f32;


pub enum Ops<T> 
where
    T: Clone
{
    Fn(&'static str, u8, Arc<dyn Fn(&[T]) -> T>),
    Value(T),
    Var(String, usize),
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
            Ops::Value(_) => "value",
            Ops::Var(name, _) => name,
            Ops::Const(name, _) => name,
            Ops::MutableConst(name, _, _, _, _) => name,
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Ops::Fn(_, arity, _) => *arity,
            Ops::Value(_) => 0,
            Ops::Var(_, _) => 0,
            Ops::Const(_, _) => 0,
            Ops::MutableConst(_, arity, _, _, _) => *arity,
        }
    }

    pub fn apply(&self, inputs: &[T]) -> T {
        match self {
            Ops::Fn(_, _, op) => op(inputs),
            Ops::Value(value) => value.clone(),
            Ops::Var(_, index) => inputs[*index].clone(),
            Ops::Const(_, value) => value.clone(),
            Ops::MutableConst(_, _, value, _, operation) => operation(inputs, value),
        }
    }

    pub fn new_instance(&self) -> Ops<T> {
        match self {
            Ops::Fn(name, arity, op) => Ops::Fn(name, *arity, op.clone()),
            Ops::Value(value) => Ops::Value(value.clone()),
            Ops::Var(name, index) => Ops::Var(name.clone(), *index),
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
            Ops::Value(value) => Ops::Value(value.clone()),
            Ops::Var(name, index) => Ops::Var(name.clone(), *index),
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
            Ops::Value(value) => write!(f, "Value: {:?}", value),
            Ops::Var(name, index) => write!(f, "Var: {}({})", name, index),
            Ops::Const(name, value) => write!(f, "Const: {}({:?})", name, value),
            Ops::MutableConst(name, arity, value, _, _) => write!(f, "MutConst: {}({})({:.2?})", name, arity, value),
        }
    }
}


pub fn clamp<T>(value: T) -> T
where
    T: Clone + Float
{
    if value.is_nan() {
        return T::from(0_f32).unwrap();
    }

    let min_value = T::from(MIN_VALUE).unwrap();
    let max_value = T::from(MAX_VALUE).unwrap();

    if value < min_value {
        min_value
    } else if value > max_value {
        max_value
    } else {
        value
    }
}


pub fn value<T: Clone>(value: T) -> Ops<T> {
    Ops::Value(value)
}

pub fn add<T: Add<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("+", 2, Arc::new(|inputs: &[T]| clamp(inputs[0].clone() + inputs[1].clone())))
}

pub fn sub<T: Sub<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("-", 2, Arc::new(|inputs: &[T]| clamp(inputs[0].clone() - inputs[1].clone())))
}

pub fn mul<T: Mul<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("*", 2, Arc::new(|inputs: &[T]| clamp(inputs[0].clone() * inputs[1].clone())))
}

pub fn div<T: Div<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("/", 2, Arc::new(|inputs: &[T]| clamp(inputs[0].clone() / inputs[1].clone())))
}

pub fn sum<T: Add<Output = T> + Clone + Default + Float>() -> Ops<T> {
    Ops::Fn("sum", 2, Arc::new(|inputs: &[T]| clamp(inputs
        .iter()
        .fold(T::default(), |acc, x| acc + x.clone()))))
}

pub fn prod<T: Mul<Output = T> + Clone + Default + Float>() -> Ops<T> {
    Ops::Fn("prod", 2, Arc::new(|inputs: &[T]| {
        let result = inputs
            .iter()
            .fold(T::default(), |acc, x| acc * x.clone());

        clamp(result)
    }))
}

pub fn neg<T: Neg<Output = T> + Clone + Default + Float>() -> Ops<T> {
    Ops::Fn("neg", 1, Arc::new(|inputs: &[T]| clamp(-inputs[0].clone())))
}

pub fn pow<T: Mul<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("pow", 2, Arc::new(|inputs: &[T]| clamp(inputs[0].clone() * inputs[1].clone())))
}

pub fn sqrt<T: Mul<Output = T> + Clone + Float>() -> Ops<T> {
    Ops::Fn("sqrt", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().sqrt())))
}

pub fn abs<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("abs", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().abs())))
}

pub fn exp<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("exp", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().exp())))
}

pub fn log<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("log", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().ln())))
}

pub fn sin<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("sin", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().sin())))
}

pub fn cos<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("cos", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().cos())))
}

pub fn tan<T: Clone + Float>() -> Ops<T> {
    Ops::Fn("tan", 1, Arc::new(|inputs: &[T]| clamp(inputs[0].clone().tan())))
}


pub fn weight<T: Sub<Output = T> + Mul<Output = T> + Copy + Default + Float>() -> Ops<T>
where
    Standard: Distribution<T>,
    T: PartialOrd + NumCast
{
    let supplier = || random::<T>() - random::<T>();
    let operation = |inputs: &[T], weight: &T| clamp(inputs[0] * *weight);
    Ops::MutableConst("w", 1, supplier(), Arc::new(supplier), Arc::new(operation))
}

pub fn var<T: Clone>(index: usize) -> Ops<T> {
    let var_name = format!("x{}", index);
    Ops::Var(var_name, index)
}

pub fn sigmoid() -> Ops<f32> {
    Ops::Fn("sigmoid", 1, Arc::new(|inputs: &[f32]| {
        let sum = inputs.iter().fold(0_f32, |acc, x| acc + x);
        let result = 1_f32 / (1_f32 + (-sum).exp());
        clamp(result)
    }))
}

pub fn relu() -> Ops<f32> {
    Ops::Fn("relu", 1, Arc::new(|inputs: &[f32]| {
        let sum = inputs.iter().fold(0_f32, |acc, x| acc + x);
        let result = clamp(sum);
        if result > 0_f32 {
            result
        } else {
            0_f32
        }
    }))
}

pub fn tanh() -> Ops<f32> {
    Ops::Fn("tanh", 1, Arc::new(|inputs: &[f32]| {
        let result = inputs
            .iter()
            .fold(0_f32, |acc, x| acc + x)
            .tanh();

        clamp(result)
    }))
}

pub fn linear() -> Ops<f32> {
    Ops::Fn("linear", 1, Arc::new(|inputs: &[f32]| {
        let result = inputs
            .iter()
            .fold(0_f32, |acc, x| acc + x);

        clamp(result)
    }))
}