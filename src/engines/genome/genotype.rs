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

    pub fn as_slice(&self) -> &[Chromosome<TGene>] {
        &self.chromosomes
    }

    pub fn as_mut_slice(&mut self) -> &mut [Chromosome<TGene>] {
        &mut self.chromosomes
    }

    pub fn get(&self, index: usize) -> &Chromosome<TGene> {
        &self.chromosomes[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Chromosome<TGene> {
        &mut self.chromosomes[index]
    }

    pub fn set(&mut self, index: usize, chromosome: Chromosome<TGene>) {
        self.chromosomes[index] = chromosome;
    }

    pub fn len(&self) -> usize {
        self.chromosomes.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Chromosome<TGene>> {
        self.chromosomes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Chromosome<TGene>> {
        self.chromosomes.iter_mut()
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