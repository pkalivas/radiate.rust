use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::alterers::{Recombinator, Alterer};
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;

pub struct UniformCrossover<TGene> where TGene: Gene<TGene>{
    pub probability: f32,
    data: std::marker::PhantomData<TGene>
}

impl<TGene> UniformCrossover<TGene> where TGene: Gene<TGene>{
    pub fn new(probability: f32) -> Self {
        UniformCrossover {
            probability,
            data: std::marker::PhantomData
        }
    }

    fn cross_chromosomes(&self, chrom_one: &mut Chromosome<TGene>, chrom_two: &mut Chromosome<TGene>) {

    }

    fn cross_genotypes(&self, geno_one: &mut Genotype<TGene>, geno_two: &mut Genotype<TGene>) {
        let chromosome_index = rand::random::<usize>() % std::cmp::min(geno_one.len(), geno_two.len());

        let mut chrom_one = geno_one.get_mut(chromosome_index);
        let mut chrom_two = geno_two.get_mut(chromosome_index);

        self.cross_chromosomes(&mut chrom_one, &mut chrom_two);
    }
}

impl<TGene> Alterer<TGene> for UniformCrossover<TGene>
    where TGene: Gene<TGene> {
        fn alter(&self, population: &mut Population<TGene>) {
            let mut parent_indexes = Vec::new();
            for _ in 0..2 {
                parent_indexes.push(rand::random::<usize>() % population.len());
            }

            parent_indexes.sort();

            self.recombine(population, &parent_indexes);
        }
}

impl<TGene> Recombinator<TGene> for UniformCrossover<TGene>
    where TGene: Gene<TGene> {    
        fn recombine(&self, population: &mut Population<TGene>, parent_indexes: &[usize]) {
            let pheno_one = population.get(parent_indexes[0]).unwrap();
            let pheno_two = population.get(parent_indexes[1]).unwrap();
    
            let mut geno_one = pheno_one.genotype.clone();
            let mut geno_two = pheno_two.genotype.clone();
    
            self.cross_genotypes(&mut geno_one, &mut geno_two);
    
            population.set(parent_indexes[0], Phenotype {
                genotype: geno_one,
                score: None
            });
    
            population.set(parent_indexes[1], Phenotype {
                genotype: geno_two,
                score: None
            });
        }
    
}

// impl<TGene> Crossover<TGene> for UniformCrossover 
//     where TGene: Gene<TGene>
// {
// }