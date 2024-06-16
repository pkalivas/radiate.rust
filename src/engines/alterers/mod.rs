use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::population::Population;
use crate::engines::genome::chromosome::Chromosome;

pub mod mutator;
pub mod uniform_crossover;
pub mod alter;

// use crate::engines::genome::genes::gene::Gene;
// use crate::engines::genome::genotype::Genotype;
// use crate::engines::genome::population::Population;
// use crate::engines::genome::chromosome::Chromosome;
// use crate::engines::genome::phenotype::Phenotype;

// pub mod mutator;
// pub mod uniform_crossover;

// pub trait Alterer<TGene>
//     where TGene : Gene<TGene>
// {
//     fn alter(&self, population: &mut Population<TGene>);
// }

// pub trait Recombinator<TGene> : Alterer<TGene>
//     where TGene : Gene<TGene>
// {
//     fn recombine(&self, population: &mut Population<TGene>, parent_indexes: &[usize]);
// }

// pub trait Crossover<TGene> : Recombinator<TGene>
//     where TGene : Gene<TGene>
// {
//     fn cross_chromosomes(&self, chrom_one: &mut Chromosome<TGene>, chrom_two: &mut Chromosome<TGene>);

//     fn cross_genotypes(&self, geno_one: &mut Genotype<TGene>, geno_two: &mut Genotype<TGene>) {
//         let chromosome_index = rand::random::<usize>() % std::cmp::min(geno_one.len(), geno_two.len());

//         let mut chrom_one = geno_one.get_mut(chromosome_index);
//         let mut chrom_two = geno_two.get_mut(chromosome_index);

//         self.cross_chromosomes(&mut chrom_one, &mut chrom_two);
//     }

//     fn recombine(&self, population: &mut Population<TGene>, parent_indexes: &[usize]) {
//         let pheno_one = population.get(parent_indexes[0]).unwrap();
//         let pheno_two = population.get(parent_indexes[1]).unwrap();

//         let mut geno_one = pheno_one.genotype.clone();
//         let mut geno_two = pheno_two.genotype.clone();

//         self.cross_genotypes(&mut geno_one, &mut geno_two);

//         population.set(parent_indexes[0], Phenotype {
//             genotype: geno_one,
//             score: None
//         });

//         population.set(parent_indexes[1], Phenotype {
//             genotype: geno_two,
//             score: None
//         });
//     }
// }

// pub trait Mutation<TGene> : Alterer<TGene>
//     where TGene : Gene<TGene>
// {
//     fn mutate_genotype(&self, genotype: &mut Genotype<TGene>);
//     fn mutate_chromosome(&self, chromosome: &mut Chromosome<TGene>);
//     fn mutate_gene(&self, gene: &TGene) -> TGene;
// }