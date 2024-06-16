use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::chromosome::Chromosome;

pub struct Genotype<TGene>
    where TGene: Gene<TGene>
{
    pub chromosomes: Vec<Chromosome<TGene>>
}

impl<TGene> Genotype<TGene>
    where TGene: Gene<TGene>
{
    pub fn from_slice(chromosomes: &[Chromosome<TGene>]) -> Self {
        Genotype { chromosomes: chromosomes.to_vec() }
    }

    pub fn from_vec(chromosomes: Vec<Chromosome<TGene>>) -> Self {
        Genotype { chromosomes }
    }
}

impl<TGene> Clone for Genotype<TGene>
    where TGene: Gene<TGene>
{
    fn clone(&self) -> Self {
        Genotype {
            chromosomes: self.chromosomes.clone()
        }
    }
}

impl<TGene> PartialEq for Genotype<TGene>
    where TGene: Gene<TGene>
{
    fn eq(&self, other: &Self) -> bool {
        self.chromosomes == other.chromosomes
    }
}

impl<TGene> std::fmt::Debug for Genotype<TGene>
    where TGene: Gene<TGene> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for chromosome in &self.chromosomes {
            write!(f, "{:?},\n ", chromosome)?;
        }
        write!(f, "]")
    }
}