use rand::Rng;

use crate::engines::alterers::mutators::mutate::Mutate;
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::NumericGene;

pub struct NumericMutator {
    rate: f32,
}

impl NumericMutator {
    pub fn new(rate: f32) -> Self {
        Self {
            rate,
        }
    }
}

impl<G: NumericGene<G, A>, A> Mutate<G, A> for NumericMutator {
    fn mutate_rate(&self) -> f32 {
        self.rate
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<G, A>, probability: f32) {
        let mut random = rand::thread_rng();

        for gene in chromosome.iter_mut() {
            if random.gen::<f32>() < probability {
                let new_instance = gene.new_instance();
                let operator = random.gen_range(0..4);

                *gene = match operator {
                    0 => gene.add(&new_instance),
                    1 => gene.sub(&new_instance),
                    2 => gene.mul(&new_instance),
                    3 => gene.div(&new_instance),
                    _ => panic!("Invalid operator: {}", operator),
                };
            }
        }
    }
}
