use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;

pub trait Alter<G: Gene<G, A>, A> {
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32);
}
