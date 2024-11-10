use crate::operations::op::Op;

pub struct VarOp<T> 
where
    T: Clone
{
    pub name: String,
    pub index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> VarOp<T> 
where
    T: Clone
{
    pub fn new(name: String, index: usize) -> VarOp<T> {
        VarOp {
            name,
            index,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Op<T> for VarOp<T> 
where
    T: Clone
{
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> u8 {
        0
    }

    fn apply(&self, inputs: &[T]) -> T {
        inputs[self.index].clone()
    }
}