use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;

pub trait Mutate<G: Gene<G, A>, A> {

    fn mutate_rate(&self) -> f32;
    
    fn mutate_genotype(&self, genotype: &mut Genotype<G, A>, range: i32, probability: f32) {
        for chromosome in genotype.iter_mut() {
            if rand::random::<i32>() > range {
                self.mutate_chromosome(chromosome, probability);
            }
        }
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<G, A>, probability: f32) {
        for gene in chromosome.iter_mut() {
            if rand::random::<f32>() < probability {
                *gene = self.mutate_gene(gene);
            }
        }
    }

    fn mutate_gene(&self, gene: &G) -> G {
        gene.new_instance()
    }
}
