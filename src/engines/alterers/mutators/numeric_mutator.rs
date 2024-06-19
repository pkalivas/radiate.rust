use rand::Rng;

use crate::engines::alterers::mutators::mutate::Mutate;
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::{Gene, NumericGene};

pub struct NumericMutator<G: NumericGene<G, A>, A> {
    rate: f32,
    _gene: std::marker::PhantomData<G>,
    _allele: std::marker::PhantomData<A>,
}

impl<G: NumericGene<G, A>, A> NumericMutator<G, A> {
    pub fn new(rate: f32) -> Self {
        Self {
            rate,
            _gene: std::marker::PhantomData,
            _allele: std::marker::PhantomData,
        }
    }
}

impl<G: NumericGene<G, A>, A> Mutate<G, A> for NumericMutator<G, A> {
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