use crate::engines::alterers::mutate::Mutate;
use crate::engines::genome::genes::gene::Gene;

pub struct Mutator {
    pub mutation_rate: f32,
}

impl Mutator {
    pub fn new(mutation_rate: f32) -> Self {
        Mutator { mutation_rate }
    }
}

impl<TGene> Mutate<TGene> for Mutator where TGene: Gene<TGene> {
    fn mutation_rate(&self) -> f32 {
        self.mutation_rate
    }
}
