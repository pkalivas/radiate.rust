use crate::engines::alterers::alter::Alter;
use crate::engines::alterers::mutate::Mutate;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;

pub struct Mutator {
    pub mutation_rate: f32,
}

impl Mutator {
    pub fn new(mutation_rate: f32) -> Self {
        Mutator { mutation_rate }
    }
}

impl<TGene> Alter<TGene> for Mutator
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>) {
        let probability = self.mutation_rate.powf(1.0 / 3.0);
        let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32) * probability)
            + (std::i32::MIN as f32)) as i32;

        for phenotype in population.iter_mut() {
            if rand::random::<i32>() > range {
                let mut genotype = phenotype.genotype().clone();

                self.mutate_genotype(&mut genotype, range, probability);

                *phenotype = Phenotype::from_genotype(genotype);
            }
        }
    }
}

impl<TGene> Mutate<TGene> for Mutator where TGene: Gene<TGene> {}
