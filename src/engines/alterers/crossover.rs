use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;

pub trait Crossover<TGene>
where
    TGene: Gene<TGene>,
{
    fn crossover_rate(&self) -> f32;

    fn cross(
        &self,
        population: &mut Population<TGene>,
        parent_indexes: &[usize],
        probability: f32,
    ) {
        let mut geno_one = population.get(parent_indexes[0]).genotype().clone();
        let mut geno_two = population.get(parent_indexes[1]).genotype().clone();

        self.cross_genotypes(&mut geno_one, &mut geno_two, probability);

        population.set(parent_indexes[0], Phenotype::from_genotype(geno_one));
        population.set(parent_indexes[1], Phenotype::from_genotype(geno_two));
    }

    fn cross_genotypes(
        &self,
        geno_one: &mut Genotype<TGene>,
        geno_two: &mut Genotype<TGene>,
        probability: f32,
    ) {
        let min_index = std::cmp::min(geno_one.len(), geno_two.len());
        let chromosome_index = rand::random::<usize>() % min_index;

        let mut chrom_one = geno_one.get_mut(chromosome_index);
        let mut chrom_two = geno_two.get_mut(chromosome_index);

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
                let gene_one = chrom_one.get(i);
                let gene_two = chrom_two.get(i);

                let new_gene_one = gene_one.from_gene(&gene_two);
                let new_gene_two = gene_two.from_gene(&gene_one);

                chrom_one.set(i, new_gene_one);
                chrom_two.set(i, new_gene_two);
            }
        }
    }
}
