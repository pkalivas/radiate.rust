use rand::Rng;

use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct SwapMutator;

impl<G, A> Mutate<G, A> for SwapMutator
where
    G: Gene<G, A>
{
    fn mutate_chromosome(&self, chromosome: &mut Chromosome<G, A>, probability: f32) {
        let mut random = rand::thread_rng();

        for i in 0..chromosome.len() {
            if random.gen::<f32>() < probability {
                let swap_index = random.gen_range(0..chromosome.len());

                if swap_index == i {
                    continue;
                }

                let curr_gene = chromosome.get_gene(i);
                let swap_gene = chromosome.get_gene(swap_index);

                chromosome.set_gene(i, curr_gene.from_allele(&swap_gene.allele()));
            }
        }
    }
}
