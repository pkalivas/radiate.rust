use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;

pub trait Mutate<TGene>
where
    TGene: Gene<TGene>,
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>, range: i32, probability: f32) {
        for chromosome in genotype.iter_mut() {
            if rand::random::<i32>() > range {
                self.mutate_chromosome(chromosome, probability);
            }
        }
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>, probability: f32) {
        for gene in chromosome.iter_mut() {
            if rand::random::<f32>() < probability {
                *gene = self.mutate_gene(gene);
            }
        }
    }

    fn mutate_gene(&self, gene: &TGene) -> TGene {
        gene.new_instance()
    }
}
