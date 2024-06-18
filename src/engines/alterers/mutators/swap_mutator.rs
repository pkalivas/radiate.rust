use rand::Rng;

use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct SwapMutator;

impl<TGene: Gene<TGene>> Mutate<TGene> for SwapMutator {
    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>, probability: f32) {
        let mut random = rand::thread_rng();

        for i in 0..chromosome.len() {
            if random.gen::<f32>() < probability {
                let swap_index = random.gen_range(0..chromosome.len());

                if swap_index == i {
                    continue;
                }

                let temp = chromosome.get_gene(i);
                let swap = chromosome.get_gene(swap_index);

                let new_gene = temp.from_gene(&swap);
                chromosome.set_gene(i, new_gene);
            }
        }
    }
}
