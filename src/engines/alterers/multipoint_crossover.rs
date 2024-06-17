use crate::engines::alterers::crossover::Crossover;
use crate::engines::genome::genes::gene::Gene;

pub struct MultiPointCrossover {
    pub probability: f32,
}

impl MultiPointCrossover {
    pub fn new(probability: f32) -> Self {
        MultiPointCrossover { probability }
    }
}

impl<TGene> Crossover<TGene> for MultiPointCrossover
where
    TGene: Gene<TGene>,
{
    fn crossover_rate(&self) -> f32 {
        self.probability
    }
}

