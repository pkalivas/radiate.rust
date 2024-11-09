use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;

pub trait Mutate<G, A> 
where
    G: Gene<G, A>
{

    fn mutate_rate(&self) -> f32;
    
    fn mutate_genotype(&self, genotype: &mut Genotype<G, A>, range: i32, probability: f32) -> i32 {
        let mut count = 0;
        for chromosome in genotype.iter_mut() {
            if rand::random::<i32>() > range {
                count += self.mutate_chromosome(chromosome, probability);
            }
        }

        count
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<G, A>, probability: f32) -> i32 {
        let mut count = 0;
        for gene in chromosome.iter_mut() {
            if rand::random::<f32>() < probability {
                *gene = self.mutate_gene(gene);
                count += 1;
            }
        }

        count
    }

    fn mutate_gene(&self, gene: &G) -> G {
        gene.new_instance()
    }
}
