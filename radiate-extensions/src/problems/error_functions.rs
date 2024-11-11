use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, SubAssign};
use num_traits::cast::FromPrimitive;
use num_traits::float::Float;

pub enum ErrorFunction {
    MSE,
    MAE,
    CrossEntropy,
    Diff
}

impl ErrorFunction {
    pub fn calculate<T>(&self, expected: &[T], actual: &[T]) -> T
    where
        T: Clone + PartialEq + Default
            + Add<Output = T> 
            + Div<Output = T> 
            + Sub<Output = T> 
            + Mul<Output = T> 
            + Div<Output = T> 
            + AddAssign 
            + SubAssign 
            + DivAssign
            + Float
            + FromPrimitive
            + DivAssign
    {
        match self {
            ErrorFunction::MSE => {
                let mut sum = T::default();
                for i in 0..expected.len() {
                    let diff = expected[i].clone() - actual[i].clone();
                    sum += diff.clone() * diff.clone();
                }
                sum /= T::from_usize(expected.len()).unwrap();
                sum
            }
            ErrorFunction::MAE => {
                let mut sum = T::default();
                for i in 0..expected.len() {
                    let diff = expected[i].clone() - actual[i].clone();
                    sum += diff.clone();
                }
                sum /= T::from_usize(expected.len()).unwrap();
                sum
            },
            ErrorFunction::CrossEntropy => {
                let mut sum = T::default();
                for i in 0..expected.len() {
                    sum += expected[i].clone() * actual[i].clone().ln();
                }
                sum
            },
            ErrorFunction::Diff => {
                let mut sum = T::default();
                for i in 0..expected.len() {
                    sum += (expected[i].clone() - actual[i].clone()).abs();
                }
                sum
            }
        }
    }
}