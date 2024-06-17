use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::population::Population;

use super::mutator::Mutator;

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
    fn cross_genotypes(&self, geno_one: &mut Genotype<TGene>, geno_two: &mut Genotype<TGene>);
    fn cross_chromosomes(&self, chrom_one: &mut Chromosome<TGene>, chrom_two: &mut Chromosome<TGene>);
}

pub trait Mutate<TGene> : Alter<TGene>
    where TGene : Gene<TGene>
{
    fn mutate_genotype(&self, genotype: &mut Genotype<TGene>);
    fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>);
    fn mutate_gene(&self, gene: &TGene) -> TGene;
}

pub enum Alterer
{
    Mutator(f32),
    UniformCrossover(f32),
    SinglePointCrossover(f32),
    TwoPointCrossover(f32),
    MultiPointCrossover(f32, usize),
}

impl Alterer
{
    pub fn mutate_rate(&self) -> f32 {
        match self {
            Alterer::Mutator(rate) => *rate,
            _ => 0.0
        }
    }

    pub fn crossover_rate(&self) -> f32 {
        match self {
            Alterer::UniformCrossover(rate) => *rate,
            Alterer::SinglePointCrossover(rate) => *rate,
            Alterer::TwoPointCrossover(rate) => *rate,
            Alterer::MultiPointCrossover(rate, _) => *rate,
            _ => 0.0
        }
    }

    pub fn multi_point_count(&self) -> usize {
        match self {
            Alterer::MultiPointCrossover(_, count) => *count,
            _ => 0
        }
    }
}

impl<TGene> Alter<TGene> for Alterer
    where TGene : Gene<TGene>
{    
    fn alter(&self, population: &mut Population<TGene>) {
        match self {
            Alterer::Mutator(rate) => {
                let mutator = crate::engines::alterers::mutator::Mutator::new(*rate);
                mutator.alter(population);
            },
            Alterer::UniformCrossover(rate) => {
                let uniform_crossover = crate::engines::alterers::uniform_crossover::UniformCrossover::new(*rate);
                uniform_crossover.alter(population);
            },
            _ => {}
            // Alterer::SinglePointCrossover(rate) => {
            //     let single_point_crossover = crate::engines::alterers::single_point_crossover::SinglePointCrossover::new(*rate);
            //     single_point_crossover.alter(population);
            // },
            // Alterer::TwoPointCrossover(rate) => {
            //     let two_point_crossover = crate::engines::alterers::two_point_crossover::TwoPointCrossover::new(*rate);
            //     two_point_crossover.alter(population);
            // },
            // Alterer::MultiPointCrossover(rate, count) => {
            //     let multi_point_crossover = crate::engines::alterers::multi_point_crossover::MultiPointCrossover::new(*rate, *count);
            //     multi_point_crossover.alter(population);
            // }
        }
    }
}