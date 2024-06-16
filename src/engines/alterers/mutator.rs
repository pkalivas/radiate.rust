use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::alterers::{Alterer, Mutation};
use crate::engines::genome::genotype::Genotype;

pub struct Mutator<TGene>
    where TGene : Gene<TGene>
{
    pub mutation_rate: f32,
    phantom: std::marker::PhantomData<TGene>
}

impl<TGene> Mutator<TGene>
    where TGene : Gene<TGene>
{
    pub fn new(mutation_rate: f32) -> Self {
        Mutator {
            mutation_rate,
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene> Alterer<TGene> for Mutator<TGene>
    where TGene : Gene<TGene>
{
    fn alter(&self, population: &Population<TGene>) -> Population<TGene> {

        unimplemented!()
    }
}

impl<TGene> Mutation<TGene> for Mutator<TGene> 
    where TGene : Gene<TGene>
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>) {
        for chromosome in &mut genotype.chromosomes {
            self.mutate_chromosome(chromosome.as_mut_slice());
        }
    }

    fn mutate_chromosome(&self, chromosome: &mut [TGene]) {
        for gene in chromosome {
            *gene = self.mutate(gene);
        }
    }

    fn mutate(&self, gene: &TGene) -> TGene {
        let mut new_gene = gene.new_instance();

        if rand::random::<f32>() < self.mutation_rate {
            new_gene = gene.new_instance();
        }

        new_gene
    }
}
