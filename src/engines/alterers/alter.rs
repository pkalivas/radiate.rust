use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::population::Population;

pub trait Alter<TGene>
    where TGene : Gene<TGene>
{
    fn alter(&self, population: &mut Population<TGene>);
}

pub trait Recombine<TGene> : Alter<TGene>
    where TGene : Gene<TGene>
{
    fn recombine(&self, population: &mut Population<TGene>, parent_indexes: &[usize]);
}

pub trait Crossover<TGene> : Recombine<TGene>
    where TGene : Gene<TGene>
{
    fn cross_chromosomes(&self, chrom_one: &mut Chromosome<TGene>, chrom_two: &mut Chromosome<TGene>);
    fn cross_genotypes(&self, geno_one: &mut Genotype<TGene>, geno_two: &mut Genotype<TGene>);
}

pub trait Mutate<TGene> : Alter<TGene>
    where TGene : Gene<TGene>
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>);
    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>);
    fn mutate_gene(&self, gene: &TGene) -> TGene;
}
