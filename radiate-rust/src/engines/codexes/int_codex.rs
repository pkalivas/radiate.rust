use crate::engines::codexes::Codex::Codex;
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::gene::{BoundGene, Gene};
use crate::engines::genome::genes::int_gene::IntGene;
use crate::engines::genome::genotype::Genotype;

pub struct IntCodex {
    pub num_chromosomes: usize,
    pub num_genes: usize,
    pub min: i32,
    pub max: i32,
    pub lower_bound: i32,
    pub upper_bound: i32,
}

impl IntCodex {
    pub fn new(num_chromosomes: usize, num_genes: usize, min: i32, max: i32) -> Self {
        IntCodex {
            num_chromosomes,
            num_genes,
            min,
            max,
            lower_bound: i32::MIN,
            upper_bound: i32::MAX,
        }
    }

    pub fn with_bounds(mut self, lower_bound: i32, upper_bound: i32) -> Self {
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
        self
    }
}

impl Codex<IntGene, i32, Vec<Vec<i32>>> for IntCodex {
    fn encode(&self) -> Genotype<IntGene, i32> {
        Genotype {
            chromosomes: (0..self.num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..self.num_genes)
                        .into_iter()
                        .map(|_| IntGene::new(self.min, self.max).with_bounds(self.lower_bound, self.upper_bound))
                        .collect::<Vec<IntGene>>()))
                .collect::<Vec<Chromosome<IntGene, i32>>>(),
        }
    }

    fn decode(&self, genotype: &Genotype<IntGene, i32>) -> Vec<Vec<i32>> {
        genotype
            .iter()
            .map(|chromosome| {
                chromosome
                    .iter()
                    .map(|gene| *gene.allele())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }
}