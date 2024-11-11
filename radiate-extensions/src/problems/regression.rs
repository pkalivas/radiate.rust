use super::{error_functions::ErrorFunction, sample_set::SampleSet};


pub struct Regression<T>{
    pub sample_set: SampleSet<T>,
    pub loss_function: ErrorFunction,
}

impl<T> Regression<T> {
    pub fn new(sample_set: SampleSet<T>, loss_function: ErrorFunction) -> Self {
        Regression { sample_set, loss_function }
    }

    pub fn from(samples: Vec<(Vec<T>, Vec<T>)>, loss_function: ErrorFunction) -> Self {
        let mut sample_set = SampleSet::new();
        for (input, output) in samples {
            sample_set.add_sample(input, output);
        }
        Regression { sample_set, loss_function }
    }

    pub fn get_sample_set(&self) -> &SampleSet<T> {
        &self.sample_set
    }

    pub fn get_loss_function(&self) -> &ErrorFunction {
        &self.loss_function
    }
}