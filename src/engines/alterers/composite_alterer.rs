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
use crate::engines::alterers::mutators::numeric_mutator::NumericMutator;

use super::crossovers::crossover::Crossover;
use super::mutators::mutate::Mutate;

pub struct CompositeAlterer<G, A>
where
    G: Gene<G, A>
{
    crossovers: Vec<Box<dyn Crossover<G, A>>>,
    mutators: Vec<Box<dyn Mutate<G, A>>>
}

impl<G, A> CompositeAlterer<G, A>
where
    G: Gene<G, A>
{
    pub fn new() -> Self {
        CompositeAlterer {
            crossovers: Vec::new(),
            mutators: Vec::new(),
        }
    }

    pub fn add_crossover(&mut self, crossover: Box<dyn Crossover<G, A>>) {
        self.crossovers.push(crossover);
    }

    pub fn add_mutator(&mut self, mutator: Box<dyn Mutate<G, A>>) {
        self.mutators.push(mutator);
    }
}

impl<G: Gene<G, A>, A> Alter<G, A> for CompositeAlterer<G, A> {
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32) {
        optimize.sort(population);

        for mutation in self.mutators.iter() {
            let rate = mutation.mutate_rate();
            let probability = rate.powf(1.0 / 3.0);
            let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32)
                * probability)
                + (std::i32::MIN as f32)) as i32;

            for phenotype in population.iter_mut() {
                if rand::random::<f32>() < probability {
                    let mut genotype = phenotype.genotype().clone();

                    mutation.mutate_genotype(&mut genotype, range, probability);

                    *phenotype = Phenotype::from_genotype(genotype, generation);
                }
            }
        }

        for crossover in self.crossovers.iter() {
            let mut random = rand::thread_rng();
            let rate = crossover.cross_rate();

            for i in 0..population.len() {
                if rand::random::<f32>() < rate {
                    let parent_indexes = subset::individual_indexes(&mut random, i, population.len(), 2);
                    crossover.cross(population, &parent_indexes, generation);
                }
            }
        }
    }
}


// impl<G: Gene<G, A>, A> Alter<G, A> for CompositeAlterer<G, A> {
//     fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32) {
//         optimize.sort(population);

//         for alterer in self.alterers.iter() {
//             match alterer.mutator {
//                 Some(ref mutator) => {
//                     let probability = alterer.rate.powf(1.0 / 3.0);
//                     let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32)
//                         * probability)
//                         + (std::i32::MIN as f32)) as i32;

//                     for phenotype in population.iter_mut() {
//                         if rand::random::<i32>() > range {
//                             let mut genotype = phenotype.genotype().clone();

//                             mutator.mutate_genotype(&mut genotype, range, probability);

//                             *phenotype = Phenotype::from_genotype(genotype, generation);
//                         }
//                     }
//                 }
//                 None => (),
//             };
//             match alterer.crossover {
//                 Some(ref crossover) => {
//                     let mut random = rand::thread_rng();

//                     for i in 0..population.len() {
//                         if rand::random::<f32>() < alterer.rate {
//                             let parent_indexes = subset::individual_indexes(&mut random, i, population.len(), 2);
//                             crossover.cross(population, &parent_indexes, alterer.rate, generation);
//                         }
//                     }
//                 }
//                 None => (),
//             };
//         }
//     }
// }
