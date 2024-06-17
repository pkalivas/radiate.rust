use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;

use super::crossovers::crossover::Crossover;
use super::mutators::mutate::Mutate;

pub trait Alter<TGene>
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>);
}

pub struct AlterWrap<TGene>
where
    TGene: Gene<TGene>
{
    pub rate: f32,
    pub mutator: Option<Box<dyn Mutate<TGene>>>,
    pub crossover: Option<Box<dyn Crossover<TGene>>>
}

pub enum Alterer {
    Mutator(f32),
    UniformCrossover(f32),
    MultiPointCrossover(f32, usize),
}