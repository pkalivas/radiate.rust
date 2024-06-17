use crate::engines::alterers::crossovers::crossover::Crossover;
use crate::engines::genome::genes::gene::Gene;

pub struct UniformCrossover;

impl<TGene> Crossover<TGene> for UniformCrossover where TGene: Gene<TGene> {}
