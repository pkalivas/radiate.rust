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
        let mut chromosomes = Vec::with_capacity(self.num_chromosomes);

        for _ in 0..self.num_chromosomes {
            let mut genes = Vec::with_capacity(self.num_genes);

            for _ in 0..self.num_genes {
                genes.push(FloatGene::new());
            }

            chromosomes.push(Chromosome::from_vec(genes));
        }

        Genotype::from_slice(&chromosomes)
    }

    fn decode(&self, genotype: &Genotype<FloatGene>) -> Vec<Vec<f32>> {
        let mut decoded = Vec::with_capacity(self.num_chromosomes);

        for chromosome in &genotype.chromosomes {
            let mut genes = Vec::with_capacity(self.num_genes);

            for gene in &chromosome.genes {
                genes.push(*gene.allele());
            }

            decoded.push(genes);
        }

        decoded
    }
}