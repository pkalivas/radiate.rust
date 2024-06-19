use crate::engines::alterers::crossovers::crossover::Crossover;
use crate::engines::alterers::mutators::mutate::Mutate;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::{self, Optimize};

pub trait Alter<G: Gene<G, A>, A> {
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32);
}

pub struct AlterWrap<G: Gene<G, A>, A> {
    pub rate: f32,
    pub mutator: Option<Box<dyn Mutate<G, A>>>,
    pub crossover: Option<Box<dyn Crossover<G, A>>>,
}

#[allow(dead_code)]
pub enum Alterer {
    Mutator(f32),
    UniformCrossover(f32),
    MultiPointCrossover(f32, usize),
    SinglePointCrossover(f32),
    SwapMutator(f32),
    NumericMutator(f32),
}
