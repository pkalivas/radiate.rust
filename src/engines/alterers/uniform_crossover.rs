use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::alterers::alter::{Recombine, Alter, Crossover};
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;

pub struct UniformCrossover {
    pub probability: f32
}

impl UniformCrossover {
    pub fn new(probability: f32) -> Self {
        UniformCrossover {
            probability,
        }
    }
}

impl<TGene> Alter<TGene> for UniformCrossover
    where TGene: Gene<TGene> 
{
    fn alter(&self, population: &mut Population<TGene>) {
        let mut parent_indexes = Vec::new();
        for _ in 0..2 {
            parent_indexes.push(rand::random::<usize>() % population.len());
        }

        parent_indexes.sort();

        self.recombine(population, &parent_indexes);
    }
}

impl<TGene> Recombine<TGene> for UniformCrossover
    where TGene: Gene<TGene> 
{    
    fn recombine(&self, population: &mut Population<TGene>, parent_indexes: &[usize]) {
        let mut geno_one = population.get(parent_indexes[0]).genotype().clone();
        let mut geno_two = population.get(parent_indexes[1]).genotype().clone();

        self.cross_genotypes(&mut geno_one, &mut geno_two);

        population.set(parent_indexes[0], Phenotype::from_genotype(geno_one));
        population.set(parent_indexes[1], Phenotype::from_genotype(geno_two));
    }
}

impl<TGene> Crossover<TGene> for UniformCrossover 
    where TGene: Gene<TGene> 
{
    fn cross_chromosomes(&self, chrom_one: &mut Chromosome<TGene>, chrom_two: &mut Chromosome<TGene>) {
        for i in 0..std::cmp::min(chrom_one.len(), chrom_two.len()) {
            if rand::random::<f32>() < self.probability {
                chrom_one.set(i, chrom_two.get(i).clone());
                chrom_two.set(i, chrom_one.get(i).clone());
            }
        }
    }
    
    fn cross_genotypes(&self, geno_one: &mut Genotype<TGene>, geno_two: &mut Genotype<TGene>) {
        let chromosome_index = rand::random::<usize>() % std::cmp::min(geno_one.len(), geno_two.len());

        let mut chrom_one = geno_one.get_mut(chromosome_index);
        let mut chrom_two = geno_two.get_mut(chromosome_index);

        self.cross_chromosomes(&mut chrom_one, &mut chrom_two);
    }
}
