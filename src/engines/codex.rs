use super::genome::{genes::gene::Gene, genotype::Genotype};
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::char_gene::CharGene;
use crate::engines::genome::genes::float_gene::FloatGene;
use crate::engines::genome::genes::int_gene::IntGene;

pub struct Codex<G, A, T>
where
    G: Gene<G, A> 
{
    pub encoder: Option<Box<dyn Fn() -> Genotype<G, A>>>,
    pub decoder: Option<fn(&Genotype<G, A>) -> T>,
}

impl<G: Gene<G, A>, A, T> Codex<G, A, T> {
    pub fn new() -> Self {
        Codex {
            encoder: None,
            decoder: None,
        }
    }

    pub fn encode(&self) -> Genotype<G, A> {
        match &self.encoder {
            Some(encoder) => encoder(),
            None => panic!("Encoder not set"),
        }
    }

    pub fn decode(&self, genotype: &Genotype<G, A>) -> T {
        match &self.decoder {
            Some(decoder) => decoder(genotype),
            None => panic!("Decoder not set"),
        }
    }

    pub fn encoder(mut self, encoder: impl Fn() -> Genotype<G, A> + 'static) -> Self {
        self.encoder = Some(Box::new(encoder));
        self
    }

    pub fn decoder(mut self, decoder: fn(&Genotype<G, A>) -> T) -> Self {
        self.decoder = Some(decoder);
        self
    }

    pub fn spawn(&self, num: i32) -> Vec<T> {
        (0..num)
            .into_iter()
            .map(|_| self.decode(&self.encode()))
            .collect::<Vec<T>>()
    }

    pub fn spawn_genotypes(&self, num: i32) -> Vec<Genotype<G, A>> {
        (0..num)
            .into_iter()
            .map(|_| self.encode())
            .collect::<Vec<Genotype<G, A>>>()
    }
}

pub fn char(num_chromosomes: usize, num_genes: usize) -> Codex<CharGene, char, String> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                        .into_iter()
                        .map(|_| CharGene::new())
                        .collect::<Vec<CharGene>>()))
                .collect::<Vec<Chromosome<CharGene, char>>>(),
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| gene.allele())
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
) -> Codex<FloatGene, f32, Vec<Vec<f32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                        .into_iter()
                        .map(|_| FloatGene::new(min, max))
                        .collect::<Vec<FloatGene>>()))
                .collect::<Vec<Chromosome<FloatGene, f32>>>()
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| gene.allele())
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
) -> Codex<IntGene, i32, Vec<Vec<i32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                        .into_iter()
                        .map(|_| IntGene::new(min, max))
                        .collect::<Vec<IntGene>>()))
                .collect::<Vec<Chromosome<IntGene, i32>>>(),
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| gene.allele())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
}
