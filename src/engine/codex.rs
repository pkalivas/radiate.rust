use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::genes::float_gene::FloatGene;
use crate::engine::genome::genotype::Genotype;
use crate::engine::genome::chromosome::Chromosome;
use crate::engine::genome::genes::gene::Allele;

pub trait Codex<TGene, T>
    where TGene : Gene<TGene>
{
    fn encode(&self) -> Genotype<TGene>;
    fn decode(&self, genotype: &Genotype<TGene>) -> T;
}


pub struct FloatCodex {
    pub num_chromosomes: usize,
    pub num_genes: usize
}

impl FloatCodex {
    pub fn new(num_chromosomes: usize, num_genes: usize) -> Self {
        FloatCodex {
            num_chromosomes,
            num_genes,
        }
    }
}

impl Codex<FloatGene, Vec<Vec<f32>>> for FloatCodex {
    fn encode(&self) -> Genotype<FloatGene> {
        Genotype { 
            chromosomes: (0..self.num_chromosomes)
                .into_iter()
                .map(|_| {
                    Chromosome {
                        genes: (0..self.num_genes)
                            .into_iter()
                            .map(|_| FloatGene::new())
                            .collect::<Vec<FloatGene>>()
                    }
                })
                .collect::<Vec<Chromosome<FloatGene>>>()
        }
    }

    fn decode(&self, genotype: &Genotype<FloatGene>) -> Vec<Vec<f32>> {
        genotype.chromosomes.iter().map(|chromosome| {
            chromosome.genes.iter().map(|gene| {
                *gene.allele()
            }).collect::<Vec<f32>>()
        }).collect::<Vec<Vec<f32>>>()
    }
}