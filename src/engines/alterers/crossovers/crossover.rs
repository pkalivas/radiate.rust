use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;

pub trait Crossover<TGene>
where
    TGene: Gene<TGene>,
{
    fn cross(&self, population: &mut Population<TGene>, parent_indexes: &[i32], probability: f32) {
        let index_one = parent_indexes[0] as usize;
        let index_two = parent_indexes[1] as usize;

        let mut geno_one = population.get(index_one).genotype().clone();
        let mut geno_two = population.get(index_two).genotype().clone();

        self.cross_genotypes(&mut geno_one, &mut geno_two, probability);

        population.set(index_one, Phenotype::from_genotype(geno_one));
        population.set(index_two, Phenotype::from_genotype(geno_two));
    }

    fn cross_genotypes(
        &self,
        geno_one: &mut Genotype<TGene>,
        geno_two: &mut Genotype<TGene>,
        probability: f32,
    ) {
        let min_index = std::cmp::min(geno_one.len(), geno_two.len());
        let chromosome_index = rand::random::<usize>() % min_index;

        let mut chrom_one = geno_one.get_chromosome_mut(chromosome_index);
        let mut chrom_two = geno_two.get_chromosome_mut(chromosome_index);

        self.cross_chromosomes(&mut chrom_one, &mut chrom_two, probability);
    }

    fn cross_chromosomes(
        &self,
        chrom_one: &mut Chromosome<TGene>,
        chrom_two: &mut Chromosome<TGene>,
        probability: f32,
    ) {
        for i in 0..std::cmp::min(chrom_one.len(), chrom_two.len()) {
            if rand::random::<f32>() < probability {
                let gene_one = chrom_one.get_gene(i);
                let gene_two = chrom_two.get_gene(i);

                let new_gene_one = gene_one.from_gene(&gene_two);
                let new_gene_two = gene_two.from_gene(&gene_one);

                chrom_one.set_gene(i, new_gene_one);
                chrom_two.set_gene(i, new_gene_two);
            }
        }
    }
}