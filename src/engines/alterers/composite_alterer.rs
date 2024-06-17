use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::alterers::alter::{Alter, Alterer, AlterWrap};
use crate::engines::alterers::mutator::Mutator;
use crate::engines::alterers::uniform_crossover::UniformCrossover;

pub struct CompositeAlterer<TGene>
where
    TGene: Gene<TGene>,
{
    alterers: Vec<AlterWrap<TGene>>,
}

impl<TGene> CompositeAlterer<TGene>
where
    TGene: Gene<TGene>,
{
    pub fn new(alterers: Vec<Alterer>) -> Self {
        let mut alterer_wraps = Vec::new();
        for alterer in alterers {
            match alterer {
                Alterer::Mutator(rate) => {
                    alterer_wraps.push(AlterWrap {
                        mutator: Some(Box::new(Mutator::new(rate))),
                        crossover: None,
                    });
                },
                Alterer::UniformCrossover(crossover) => {
                    alterer_wraps.push(AlterWrap {
                        mutator: None,
                        crossover: Some(Box::new(UniformCrossover::new(crossover))),
                    });
                },
            }
        }

        CompositeAlterer { alterers: alterer_wraps }
    }
}

impl<TGene> Alter<TGene> for CompositeAlterer<TGene>
where
    TGene: Gene<TGene>,
{
    fn alter(&self, population: &mut Population<TGene>) {
        for alterer in self.alterers.iter() {
            match alterer.mutator {
                Some(ref mutator) => {
                    let probability = mutator.mutation_rate().powf(1.0 / 3.0);
                    let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32) * probability)
                        + (std::i32::MIN as f32)) as i32;
            
                    for phenotype in population.iter_mut() {
                        if rand::random::<i32>() > range {
                            let mut genotype = phenotype.genotype().clone();
            
                            mutator.mutate_genotype(&mut genotype, range, probability);
            
                            *phenotype = Phenotype::from_genotype(genotype);
                        }
                    }
                },
                None => (),
            };
            match alterer.crossover {
                Some(ref crossover) => {
                    let mut parent_indexes = Vec::new();
                    for _ in 0..2 {
                        parent_indexes.push(rand::random::<usize>() % population.len());
                    }
            
                    parent_indexes.sort();
            
                    crossover.cross(population, &parent_indexes, crossover.crossover_rate());
                },
                None => (),
            };
        }
    }
}