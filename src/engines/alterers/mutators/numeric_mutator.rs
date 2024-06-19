use rand::Rng;

use crate::engines::alterers::mutators::mutate::Mutate;
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::{Gene, NumericGene};

pub struct NumericMutator;

impl<TGene: NumericGene<TGene>> Mutate<TGene> for NumericMutator {
    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>, probability: f32) {
        let mut random = rand::thread_rng();

        for gene in chromosome.iter_mut() {
            if random.gen::<f32>() < probability {
                let new_instance = gene.new_instance();
                let operator = random.gen_range(0..4);

                // gene += new_instance;

                // *gene = match operator {
                //     0 => gene + new_instance,
                //     1 => gene - new_instance,
                //     2 => gene * new_instance,
                //     3 => gene / new_instance,
                //     _ => panic!("Invalid operator"),
                // }
            }
        }
    }
}