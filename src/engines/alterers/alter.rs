use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;

pub trait Alter<G, A>
where
    G: Gene<G, A>
{
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32);
}