use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::genotype::Genotype;
use crate::engine::genome::chromosome::Chromosome;
use crate::engine::score::Score;

pub struct Phenotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub genotype: Genotype<TGene, T>,
    pub score: Option<Score>,
    phantom: std::marker::PhantomData<T>
}

impl<TGene, T> Phenotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub fn genotype(&self) -> &Genotype<TGene, T> {
        &self.genotype
    }

    pub fn genotype_mut(&mut self) -> &mut Genotype<TGene, T> {
        &mut self.genotype
    }

    pub fn from_genotype(genotype: Genotype<TGene, T>) -> Self {
        Phenotype {
            genotype,
            score: None,
            phantom: std::marker::PhantomData
        }
    }

    pub fn from_vec(chromosomes: Vec<Chromosome<TGene, T>>) -> Self {
        Phenotype {
            genotype: Genotype::from_vec(chromosomes),
            score: None,
            phantom: std::marker::PhantomData
        }
    }

    pub fn from_slice(chromosomes: &[Chromosome<TGene, T>]) -> Self {
        Phenotype {
            genotype: Genotype::from_slice(chromosomes),
            score: None,
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene, T> Clone for Phenotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn clone(&self) -> Self {
        Phenotype {
            genotype: self.genotype.clone(),
            score: self.score.clone(),
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene, T> PartialEq for Phenotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn eq(&self, other: &Self) -> bool {
        self.genotype == other.genotype && self.score == other.score
    }
}

impl<TGene, T> PartialOrd for Phenotype<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl <TGene, T> std::fmt::Debug for Phenotype<TGene, T>
    where TGene: Gene<TGene, T> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, score: {:?}", self.genotype, self.score)
    }
}