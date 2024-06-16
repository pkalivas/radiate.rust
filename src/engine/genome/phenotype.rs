use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::genotype::Genotype;
use crate::engine::genome::chromosome::Chromosome;
use crate::engine::score::Score;

pub struct Phenotype<TGene>
    where TGene: Gene<TGene>
{
    pub genotype: Genotype<TGene>,
    pub score: Option<Score>,
}

impl<TGene> Phenotype<TGene>
    where TGene: Gene<TGene>
{
    pub fn genotype(&self) -> &Genotype<TGene> {
        &self.genotype
    }

    pub fn genotype_mut(&mut self) -> &mut Genotype<TGene> {
        &mut self.genotype
    }

    pub fn from_genotype(genotype: Genotype<TGene>) -> Self {
        Phenotype {
            genotype,
            score: None,
        }
    }

    pub fn from_vec(chromosomes: Vec<Chromosome<TGene>>) -> Self {
        Phenotype {
            genotype: Genotype::from_vec(chromosomes),
            score: None
        }
    }

    pub fn from_slice(chromosomes: &[Chromosome<TGene>]) -> Self {
        Phenotype {
            genotype: Genotype::from_slice(chromosomes),
            score: None
        }
    }
}

impl<TGene> Clone for Phenotype<TGene>
    where TGene: Gene<TGene>
{
    fn clone(&self) -> Self {
        Phenotype {
            genotype: self.genotype.clone(),
            score: self.score.clone()
        }
    }
}

impl<TGene> PartialEq for Phenotype<TGene>
    where TGene: Gene<TGene>
{
    fn eq(&self, other: &Self) -> bool {
        self.genotype == other.genotype && self.score == other.score
    }
}

impl<TGene> PartialOrd for Phenotype<TGene>
    where TGene: Gene<TGene>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl <TGene> std::fmt::Debug for Phenotype<TGene>
    where TGene: Gene<TGene> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, score: {:?}", self.genotype, self.score)
    }
}