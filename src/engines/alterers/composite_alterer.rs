use crate::engines::alterers::alter::{Alter, AlterWrap, Alterer};
use crate::engines::alterers::crossovers::multipoint_crossover::MultiPointCrossover;
use crate::engines::alterers::crossovers::uniform_crossover::UniformCrossover;
use crate::engines::alterers::mutators::mutator::Mutator;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::schema::subset;
use crate::engines::alterers::mutators::swap_mutator::SwapMutator;

pub struct CompositeAlterer<TGene, A>
where
    TGene: Gene<TGene, A>
{
    alterers: Vec<AlterWrap<TGene, A>>,
}

impl<TGene, A> CompositeAlterer<TGene, A>
where
    TGene: Gene<TGene, A>
{
    pub fn new(alterers: Vec<Alterer>) -> Self {
        let mut alterer_wraps = Vec::new();
        for alterer in alterers {
            match alterer {
                Alterer::Mutator(rate) => {
                    alterer_wraps.push(AlterWrap {
                        rate,
                        mutator: Some(Box::new(Mutator)),
                        crossover: None,
                    });
                }
                Alterer::UniformCrossover(rate) => {
                    alterer_wraps.push(AlterWrap {
                        rate,
                        mutator: None,
                        crossover: Some(Box::new(UniformCrossover)),
                    });
                }
                Alterer::SinglePointCrossover(rate) => {
                    alterer_wraps.push(AlterWrap {
                        rate,
                        mutator: None,
                        crossover: Some(Box::new(MultiPointCrossover::new(1))),
                    });
                }
                Alterer::MultiPointCrossover(rate, num_points) => {
                    alterer_wraps.push(AlterWrap {
                        rate,
                        mutator: None,
                        crossover: Some(Box::new(MultiPointCrossover::new(num_points))),
                    });
                }
                Alterer::SwapMutator(rate) => {
                    alterer_wraps.push(AlterWrap {
                        rate,
                        mutator: Some(Box::new(SwapMutator)),
                        crossover: None,
                    });
                }
                Alterer::NumericMutator(rate) => panic!("NumericMutator not implemented")
            }
        }

        CompositeAlterer {
            alterers: alterer_wraps,
        }
    }
}

impl<TGene: Gene<TGene, A>, A> Alter<TGene, A> for CompositeAlterer<TGene, A> {
    fn alter(&self, population: &mut Population<TGene, A>, optimize: &Optimize, generation: i32) {
        optimize.sort(population);

        for alterer in self.alterers.iter() {
            match alterer.mutator {
                Some(ref mutator) => {
                    let probability = alterer.rate.powf(1.0 / 3.0);
                    let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32)
                        * probability)
                        + (std::i32::MIN as f32)) as i32;

                    for phenotype in population.iter_mut() {
                        if rand::random::<i32>() > range {
                            let mut genotype = phenotype.genotype().clone();

                            mutator.mutate_genotype(&mut genotype, range, probability);

                            *phenotype = Phenotype::from_genotype(genotype, generation);
                        }
                    }
                }
                None => (),
            };
            match alterer.crossover {
                Some(ref crossover) => {
                    let mut random = rand::thread_rng();

                    for i in 0..population.len() {
                        if rand::random::<f32>() < alterer.rate {
                            let parent_indexes = subset::individual_indexes(&mut random, i, population.len(), 2);
                            crossover.cross(population, &parent_indexes, alterer.rate, generation);
                        }
                    }
                }
                None => (),
            };
        }
    }
}
