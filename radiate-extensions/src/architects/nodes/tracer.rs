
pub struct Tracer<T>
where
    T: Clone
{
    pub input_size: usize,
    pub pending_idx: usize,
    pub args: Vec<T>,
    pub result: Option<T>,
    pub previous_result: Option<T>,
}

impl<T> Tracer<T> 
where
    T: Clone
{
    pub fn new(input_size: usize) -> Self {
        Tracer {
            input_size,
            pending_idx: 0,
            args: Vec::with_capacity(input_size),
            result: None,
            previous_result: None,
        }
    }
}

impl<T> Clone for Tracer<T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        Tracer {
            input_size: self.input_size,
            pending_idx: self.pending_idx,
            args: self.args.clone(),
            result: self.result.clone(),
            previous_result: self.previous_result.clone(),
        }
    }
}