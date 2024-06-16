use crate::engine::genome::genes::gene::Gene;

use super::genome::genotype::Genotype;

pub trait Codex<TGene, T>
    where TGene : Gene<TGene>
{
    fn encode(&self) -> Genotype<TGene>;
    fn decode(&self, genotype: Genotype<TGene>) -> T;
}
