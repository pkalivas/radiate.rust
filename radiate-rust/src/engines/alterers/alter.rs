use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;

use super::crossovers::crossover::Crossover;
use super::mutators::mutate::Mutate;

pub trait Alter<G, A>
where
    G: Gene<G, A>
{
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32);
}


pub struct AlterWrap<G, A> 
where
    G: Gene<G, A>
{
    pub rate: f32,
    pub mutator: Option<Box<dyn Mutate<G, A>>>,
    pub crossover: Option<Box<dyn Crossover<G, A>>>,
    pub alterer: Option<Box<dyn Alter<G, A>>>,
}


pub enum Alterer<G, A> 
where
    G: Gene<G, A>
{
    Mutator(f32),
    UniformCrossover(f32),
    MultiPointCrossover(f32, usize),
    SinglePointCrossover(f32),
    SwapMutator(f32),
    Mutation(Box<dyn Mutate<G, A>>),
    Crossover(Box<dyn Crossover<G, A>>),
    Alterer(Box<dyn Alter<G, A>>)
}