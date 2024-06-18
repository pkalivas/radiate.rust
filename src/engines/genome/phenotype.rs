use crate::engines::score::Score;

use super::{genes::gene::Gene, genotype::Genotype};

pub struct Phenotype<TGene: Gene<TGene>> {
    pub genotype: Genotype<TGene>,
    pub score: Option<Score>,
    pub generation: i32,
}

impl<TGene: Gene<TGene>> Phenotype<TGene> {
    pub fn genotype(&self) -> &Genotype<TGene> {
        &self.genotype
    }

    pub fn from_genotype(genotype: Genotype<TGene>, generation: i32) -> Self {
        Phenotype {
            genotype,
            score: None,
            generation
        }
    }

    pub fn score(&self) -> &Option<Score> {
        &self.score
    }

    pub fn set_score(&mut self, score: Option<Score>) {
        self.score = score;
    }

    pub fn age(&self, generation: i32) -> i32 {
        generation - self.generation
    }
}

impl<TGene: Gene<TGene>> Clone for Phenotype<TGene> {
    fn clone(&self) -> Self {
        Phenotype {
            genotype: self.genotype.clone(),
            score: match &self.score {
                Some(score) => Some(score.clone()),
                None => None,
            },
            generation: self.generation
        }
    }
}

impl<TGene: Gene<TGene>> PartialEq for Phenotype<TGene> {
    fn eq(&self, other: &Self) -> bool {
        self.genotype == other.genotype && self.score == other.score && self.generation == other.generation
    }
}

impl<TGene: Gene<TGene>> PartialOrd for Phenotype<TGene> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<TGene: Gene<TGene> + std::fmt::Debug> std::fmt::Debug for Phenotype<TGene> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, generation: {:?}, score: {:?}", self.genotype, self.generation, self.score)
    }
}
