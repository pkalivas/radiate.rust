use super::{loss_functions::LossFunction, sample_set::SampleSet};


pub struct Regression<T>{
    pub sample_set: SampleSet<T>,
    pub loss_function: LossFunction,
}