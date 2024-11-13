use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;

pub trait Mutate<G, A>
where
    G: Gene<G, A>
{
    fn mutate_rate(&self) -> f32;
    
    fn mutate_genotype(&self, genotype: &mut Genotype<G, A>, range: i32) -> i32 {
        let mut count = 0;
        for chromosome in genotype.iter_mut() {
            if rand::random::<i32>() > range {
                count += self.mutate_chromosome(chromosome, range);
            }
        }

        count
    }

    fn mutate_chromosome(&self, chromosome: &mut Chromosome<G, A>, range: i32) -> i32 {
        let mut count = 0;
        for gene in chromosome.iter_mut() {
            if rand::random::<i32>() > range {
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


    // fn alter(&self, population: &mut Population<G, A>, _: &Optimize, generation: i32) {
    //     let probability = self.mutate_rate().powf(1.0 / 3.0);
    //     let range = ((((std::i32::MAX as i64 - (std::i32::MIN as i64)) as f32) * probability)
    //         + (std::i32::MIN as f32)) as i32;

    //     for phenotype in population.iter_mut() {
    //         if rand::random::<i32>() > range {
    //             let mut genotype = phenotype.genotype().clone();

    //             let mutation_count = self.mutate_genotype(&mut genotype, range);

    //             if mutation_count > 0 {
    //                 *phenotype = Phenotype::from_genotype(genotype, generation);
    //             }
    //         }
    //     }
    // }