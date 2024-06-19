use crate::engines::alterers::crossovers::crossover::Crossover;
use crate::engines::genome::genes::gene::Gene;

pub struct UniformCrossover;

impl<G: Gene<G, A>, A> Crossover<G, A> for UniformCrossover {}
