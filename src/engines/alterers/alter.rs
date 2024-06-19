use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::schema::subset;

use super::crossovers::crossover::Crossover;
use super::mutators::mutate::Mutate;
use super::mutators::numeric_mutator::NumericMutator;

pub trait Alter<G, A>
where
    G: Gene<G, A>
{
    fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32);
}



// pub enum TempAlter {
//     Mutator(f32),
//     UniformCrossover(f32),
//     NumericMutator(f32),
//     MultiPointCrossover(f32, u8),
//     SwapMutator(f32)
// }

// pub struct AlterWrapper<G, A>
// where
//     G: Gene<G, A> 
// {
//     pub mutator: Option<Box<dyn Mutate<G, A>>>,
//     pub crossover: Option<Box<dyn Crossover<G, A>>>
// }

// impl<G, A> AlterWrapper<G, A>
// where
//     G: Gene<G, A>
// {
//     pub fn new() -> Self {
//         AlterWrapper {
//             mutator: None,
//             crossover: None
//         }
//     }

//     pub fn add_mutator(&mut self, mutator: Box<dyn Mutate<G, A>>) {
//         self.mutator = Some(mutator);
//     }

//     pub fn add_crossover(&mut self, crossover: Box<dyn Crossover<G, A>>) {
//         self.crossover = Some(crossover);
//     }
// }

// impl<G, A> Alter<G, A> for AlterWrapper<G, A>
// where
//     G: Gene<G, A>
// {
//     fn alter(&self, population: &mut Population<G, A>, optimize: &Optimize, generation: i32) {
//         optimize.sort(population);

//         let mut random = rand::thread_rng();

//         if let Some(crossover) = &self.crossover {
//             let rate = crossover.cross_rate();

//             for i in 0..population.len() {
//                 if rand::random::<f32>() < rate {
//                     let parent_indexes = subset::individual_indexes(&mut random, i, population.len(), 2);
//                     crossover.cross(population, &parent_indexes, generation);
//                 }
//             }
//         }

//         if let Some(mutator) = &self.mutator {
//             let rate = mutator.mutate_rate();
//             let probability = rate.powf(1.0 / 3.0);
//             let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32)
//                 * probability)
//                 + (std::i32::MIN as f32)) as i32;

//             for phenotype in population.iter_mut() {
//                 if rand::random::<f32>() < probability {
//                     let mut genotype = phenotype.genotype().clone();

//                     mutator.mutate_genotype(&mut genotype, range, probability);

//                     *phenotype = Phenotype::from_genotype(genotype, generation);
//                 }
//             }
//         }
//     }
// }
