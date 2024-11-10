use super::op::Op;


pub enum Operation<T> 
where
    T: Clone
{
    Fn(&'static str, u8, Box<dyn Fn(&[T]) -> T>),
    Math(&'static str, u8, Box<dyn Fn(&[T]) -> T>),
    Var(String, usize),
    Const(&'static str, T),
    MutableConst(&'static str, u8, T, Box<dyn Fn() -> T>, Box<dyn Fn(&[T], &T) -> T>),
}

impl<T> Operation<T>
where
    T: Clone
{
    pub fn name(&self) -> &str {
        match self {
            Operation::Fn(name, _, _) => name,
            Operation::Math(name, _, _) => name,
            Operation::Var(name, _) => name,
            Operation::Const(name, _) => name,
            Operation::MutableConst(name, _, _, _, _) => name,
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Operation::Fn(_, arity, _) => *arity,
            Operation::Math(_, arity, _) => *arity,
            Operation::Var(_, _) => 0,
            Operation::Const(_, _) => 0,
            Operation::MutableConst(_, arity, _, _, _) => *arity,
        }
    }

    pub fn apply(&self, inputs: &[T]) -> T {
        match self {
            Operation::Fn(_, _, op) => op(inputs),
            Operation::Math(_, _, op) => op(inputs),
            Operation::Var(_, index) => inputs[*index].clone(),
            Operation::Const(_, value) => value.clone(),
            Operation::MutableConst(_, _, value, _, operation) => operation(inputs, value),
        }
    }
}

impl<T> Op<T> for Operation<T>
where
    T: Clone
{
    fn name(&self) -> &str {
        self.name()
    }

    fn arity(&self) -> u8 {
        self.arity()
    }

    fn apply(&self, inputs: &[T]) -> T {
        self.apply(inputs)
    }
}