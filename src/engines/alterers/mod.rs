use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::population::Population;

mod mutator;

pub trait Alterer<TGene>
    where TGene : Gene<TGene>
{
    fn alter(&self, population: &Population<TGene>) -> Population<TGene>;
}

pub trait Mutation<TGene> : Alterer<TGene>
    where TGene : Gene<TGene>
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>);
    fn mutate_chromosome(&self, chromosome: &mut [TGene]);
    fn mutate(&self, gene: &TGene) -> TGene;
}