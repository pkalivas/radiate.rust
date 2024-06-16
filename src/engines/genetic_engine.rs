use crate::engines::genome::genes::gene::Gene;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::engine::Engine;

pub struct GeneticEngine<TGene, T> 
    where TGene : Gene<TGene>
{
    pub params: GeneticEngineParams<TGene, T>,
}

impl<TGene, T> GeneticEngine<TGene, T>
    where TGene : Gene<TGene>
{
    pub fn new(params: GeneticEngineParams<TGene, T>) -> Self {
        GeneticEngine { params }
    }
}

impl<TGene, T> Engine for GeneticEngine<TGene, T>
    where TGene : Gene<TGene>
{
    fn run(&self) {
        unimplemented!()
    }
}