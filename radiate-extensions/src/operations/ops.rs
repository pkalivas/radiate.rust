use std::{fmt::Debug, sync::Arc};


pub enum Ops<T> 
where
    T: Clone
{
    Fn(&'static str, u8, Arc<dyn Fn(&[T]) -> T>),
    Math(&'static str, u8, Arc<dyn Fn(&[T]) -> T>),
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
            Ops::Math(name, _, _) => name,
            Ops::Var(name, _) => name,
            Ops::Const(name, _) => name,
            Ops::MutableConst(name, _, _, _, _) => name,
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Ops::Fn(_, arity, _) => *arity,
            Ops::Math(_, arity, _) => *arity,
            Ops::Var(_, _) => 0,
            Ops::Const(_, _) => 0,
            Ops::MutableConst(_, arity, _, _, _) => *arity,
        }
    }

    pub fn apply(&self, inputs: &[T]) -> T {
        match self {
            Ops::Fn(_, _, op) => op(inputs),
            Ops::Math(_, _, op) => op(inputs),
            Ops::Var(_, index) => inputs[*index].clone(),
            Ops::Const(_, value) => value.clone(),
            Ops::MutableConst(_, _, value, _, operation) => operation(inputs, value),
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
            Ops::Math(name, arity, op) => Ops::Math(name, *arity, op.clone()),
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

impl<T> Debug for Ops<T>
where
    T: Clone + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ops::Fn(name, arity, _) => write!(f, "Fn: {}({})", name, arity),
            Ops::Math(name, arity, _) => write!(f, "Math: {}({})", name, arity),
            Ops::Var(name, index) => write!(f, "Var: {}({})", name, index),
            Ops::Const(name, value) => write!(f, "Const: {}({:?})", name, value),
            Ops::MutableConst(name, arity, value, _, _) => write!(f, "MutableConst: {}({})({:?})", name, arity, value),
        }
    }
}