use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::chromosome::Chromosome;

pub struct Genotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub chromosomes: Vec<Chromosome<TGene, T>>
}

impl<TGene, T> Genotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub fn from_slice(chromosomes: &[Chromosome<TGene, T>]) -> Self {
        Genotype { chromosomes: chromosomes.to_vec() }
    }

    pub fn from_vec(chromosomes: Vec<Chromosome<TGene, T>>) -> Self {
        Genotype { chromosomes }
    }
}

impl<TGene, T> Clone for Genotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn clone(&self) -> Self {
        Genotype {
            chromosomes: self.chromosomes.clone()
        }
    }
}

impl<TGene, T> PartialEq for Genotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn eq(&self, other: &Self) -> bool {
        self.chromosomes == other.chromosomes
    }
}

impl<TGene, T> std::fmt::Debug for Genotype<TGene, T>
    where TGene: Gene<TGene, T> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for chromosome in &self.chromosomes {
            write!(f, "{:?},\n ", chromosome)?;
        }
        write!(f, "]")
    }
}