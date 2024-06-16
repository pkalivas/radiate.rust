use crate::engine::genome::genes::gene::Gene;

use super::genome::genotype::Genotype;

pub trait Codex<TGene, T, TK>
    where TGene : Gene<TGene, T>
{
    fn encode(&self) -> Genotype<TGene, T>;
    fn decode(&self, genotype: Genotype<TGene, T>) -> TK;
}
