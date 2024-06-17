use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;

pub trait Alter<TGene>
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>);
}

