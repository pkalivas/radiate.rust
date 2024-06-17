use crate::engines::alterers::alter::Alter;
use crate::engines::alterers::crossover::Crossover;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;

pub struct UniformCrossover {
    pub probability: f32,
}

impl UniformCrossover {
    pub fn new(probability: f32) -> Self {
        UniformCrossover { probability }
    }
}

impl<TGene> Alter<TGene> for UniformCrossover
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>) {
        let mut parent_indexes = Vec::new();
        for _ in 0..2 {
            parent_indexes.push(rand::random::<usize>() % population.len());
        }

        parent_indexes.sort();

        self.cross(population, &parent_indexes, self.probability);
    }
}

impl<TGene> Crossover<TGene> for UniformCrossover where TGene: Gene<TGene> {}
