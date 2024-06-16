use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::alterers::alter::{Alter, Mutate};
use crate::engines::genome::genotype::Genotype;

pub struct Mutator {
    pub mutation_probability: f32,
    pub scaled_range: i32,
}

impl Mutator {
    pub fn new(mutation_rate: f32) -> Self {
        Mutator {
            mutation_probability: mutation_rate.powf(1.0 / 3.0),
            scaled_range: (std::i32::MAX as i64 - (std::i32::MIN as i64)) as i32,
        }
    }
}

impl<TGene> Alter<TGene> for Mutator
    where TGene : Gene<TGene>
{
    fn alter(&self, population: &mut Population<TGene>) {
        for phenotype in population.iter_mut() {
            if rand::random::<i32>() > self.scaled_range {
                let mut genotype = phenotype.genotype.clone();

                self.mutate_genotype(&mut genotype);

                *phenotype = Phenotype {
                    genotype,
                    score: None
                };
            }
        }
    }
}

impl<TGene> Mutate<TGene> for Mutator
    where TGene : Gene<TGene>
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>) {
        for chromosome in genotype.iter_mut() {
            if rand::random::<i32>() > self.scaled_range { 
                self.mutate_chromosome(chromosome);
            }
        }
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>) {
        for gene in chromosome.as_mut_slice() {
            if rand::random::<f32>() < self.mutation_probability {
                *gene = self.mutate_gene(gene);
            }
        }
    }

    fn mutate_gene(&self, gene: &TGene) -> TGene {
        gene.new_instance()
    }
}
