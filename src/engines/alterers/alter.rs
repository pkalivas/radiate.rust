use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::alterers::crossovers::crossover::Crossover;
use crate::engines::alterers::mutators::mutate::Mutate;
use crate::engines::optimize::Optimize;

pub trait Alter<TGene>
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>, optimize: &Optimize);
}

pub struct AlterWrap<TGene>
where
    TGene: Gene<TGene>
{
    pub rate: f32,
    pub mutator: Option<Box<dyn Mutate<TGene>>>,
    pub crossover: Option<Box<dyn Crossover<TGene>>>
}

#[allow(dead_code)]
pub enum Alterer {
    Mutator(f32),
    UniformCrossover(f32),
    MultiPointCrossover(f32, usize),
    SinglePointCrossover(f32),
    SwapMutator(f32)
}