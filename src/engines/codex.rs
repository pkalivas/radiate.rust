use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::char_gene::CharGene;
use crate::engines::genome::genes::float_gene::FloatGene;
use crate::engines::genome::genes::gene::Allele;
use crate::engines::genome::genes::int_gene::IntGene;
use super::genome::{genes::gene::Gene, genotype::Genotype};

pub struct Codex<TGene: Gene<TGene>, T> {
    pub encoder: Option<Box<dyn Fn() -> Genotype<TGene>>>,
    pub decoder: Option<fn(&Genotype<TGene>) -> T>,
}

impl<TGene: Gene<TGene>, T> Codex<TGene, T> {
    pub fn new() -> Self {
        Codex {
            encoder: None,
            decoder: None,
        }
    }

    pub fn encode(&self) -> Genotype<TGene> {
        match &self.encoder {
            Some(encoder) => encoder(),
            None => panic!("Encoder not set"),
        }
    }

    pub fn decode(&self, genotype: &Genotype<TGene>) -> T {
        match &self.decoder {
            Some(decoder) => decoder(genotype),
            None => panic!("Decoder not set"),
        }
    }

    pub fn encoder(mut self, encoder: impl Fn() -> Genotype<TGene> + 'static) -> Self {
        self.encoder = Some(Box::new(encoder));
        self
    }

    pub fn decoder(mut self, decoder: fn(&Genotype<TGene>) -> T) -> Self {
        self.decoder = Some(decoder);
        self
    }

    pub fn spawn(&self, num: i32) -> Vec<T> {
        (0..num)
            .into_iter()
            .map(|_| self.decode(&self.encode()))
            .collect::<Vec<T>>()
    }

    pub fn spawn_genotypes(&self, num: i32) -> Vec<Genotype<TGene>> {
        (0..num)
            .into_iter()
            .map(|_| self.encode())
            .collect::<Vec<Genotype<TGene>>>()
    }
}

pub fn char(num_chromosomes: usize, num_genes: usize) -> Codex<CharGene, String> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome {
                    genes: (0..num_genes)
                        .into_iter()
                        .map(|_| CharGene::new())
                        .collect::<Vec<CharGene>>(),
                })
                .collect::<Vec<Chromosome<CharGene>>>(),
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| *gene.allele())
                        .collect::<String>()
                })
                .collect::<String>()
        })
}

pub fn float(
    num_chromosomes: i32,
    num_genes: i32,
    min: f32,
    max: f32,
) -> Codex<FloatGene, Vec<Vec<f32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome {
                    genes: (0..num_genes)
                        .into_iter()
                        .map(|_| FloatGene::new(min, max))
                        .collect::<Vec<FloatGene>>(),
                })
                .collect::<Vec<Chromosome<FloatGene>>>(),
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| *gene.allele())
                        .collect::<Vec<f32>>()
                })
                .collect::<Vec<Vec<f32>>>()
        })
}

pub fn int(
    num_chromosomes: i32,
    num_genes: i32,
    max: i32,
    min: i32,
) -> Codex<IntGene, Vec<Vec<i32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome {
                    genes: (0..num_genes)
                        .into_iter()
                        .map(|_| IntGene::new(min, max))
                        .collect::<Vec<IntGene>>(),
                })
                .collect::<Vec<Chromosome<IntGene>>>(),
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| *gene.allele())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
}
