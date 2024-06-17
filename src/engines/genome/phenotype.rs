use crate::engines::score::Score;

use super::{genes::gene::Gene, genotype::Genotype};


pub struct Phenotype<TGene>
where
    TGene: Gene<TGene>,
{
    pub genotype: Genotype<TGene>,
    pub score: Option<Score>,
}

impl<TGene> Phenotype<TGene>
where
    TGene: Gene<TGene>,
{
    pub fn genotype(&self) -> &Genotype<TGene> {
        &self.genotype
    }

    pub fn from_genotype(genotype: Genotype<TGene>) -> Self {
        Phenotype {
            genotype,
            score: None,
        }
    }

    pub fn score(&self) -> &Option<Score> {
        &self.score
    }

    pub fn set_score(&mut self, score: Score) {
        self.score = Some(score);
    }
}

impl<TGene> Clone for Phenotype<TGene>
where
    TGene: Gene<TGene>,
{
    fn clone(&self) -> Self {
        Phenotype {
            genotype: self.genotype.clone(),
            score: match &self.score {
                Some(score) => Some(score.clone()),
                None => None,
            },
        }
    }
}

impl<TGene> PartialEq for Phenotype<TGene>
where
    TGene: Gene<TGene>,
{
    fn eq(&self, other: &Self) -> bool {
        self.genotype == other.genotype && self.score == other.score
    }
}

impl<TGene> PartialOrd for Phenotype<TGene>
where
    TGene: Gene<TGene>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<TGene> std::fmt::Debug for Phenotype<TGene>
where
    TGene: Gene<TGene> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, score: {:?}", self.genotype, self.score)
    }
}

