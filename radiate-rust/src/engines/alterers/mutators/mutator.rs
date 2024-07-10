use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct Mutator {
    pub rate: f32,
}

impl Mutator {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl<G, A> Mutate<G, A> for Mutator where G: Gene<G, A> {
    fn mutate_rate(&self) -> f32 {
        self.rate
    }
}
