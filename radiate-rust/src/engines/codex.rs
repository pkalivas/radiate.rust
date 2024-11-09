use std::sync::Arc;

use super::genome::{genes::gene::Gene, genotype::Genotype};
use crate::engines::genome::chromosome::Chromosome;
use crate::engines::genome::genes::bit_gene::BitGene;
use crate::engines::genome::genes::char_gene::CharGene;
use crate::engines::genome::genes::float_gene::FloatGene;
use crate::engines::genome::genes::gene::BoundGene;
use crate::engines::genome::genes::int_gene::IntGene;
use crate::engines::genome::population::Population;
use crate::engines::genome::phenotype::Phenotype;


pub struct Codex<G, A, T>
where
    G: Gene<G, A>
{
    pub encoder: Option<Arc<dyn Fn() -> Genotype<G, A>>>,
    pub decoder: Option<Arc<dyn Fn(&Genotype<G, A>) -> T>>,
}

impl<G, A, T> Codex<G, A, T> 
where 
    G: Gene<G, A>
{
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
        self.encoder = Some(Arc::new(encoder));
        self
    }

    pub fn decoder(mut self, decoder: impl Fn(&Genotype<G, A>) -> T + 'static) -> Self {
        self.decoder = Some(Arc::new(decoder));
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

    pub fn spawn_population(&self, num: i32) -> Population<G, A> {
        (0..num)
            .into_iter()
            .map(|_| Phenotype::from_genotype(self.encode(), 0))
            .collect::<Population<G, A>>()
    }
}


/// char codex
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


/// float codex
pub fn float(
    num_chromosomes: i32,
    num_genes: i32,
    min: f32,
    max: f32,
) -> Codex<FloatGene, f32, Vec<Vec<f32>>>
{
    float_with_bounds(num_chromosomes, num_genes, min, max, f32::MIN, f32::MAX)
}

pub fn float_with_bounds(
    num_chromosomes: i32,
    num_genes: i32,
    min: f32,
    max: f32,
    lower_bound: f32,
    upper_bound: f32,
) -> Codex<FloatGene, f32, Vec<Vec<f32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                        .into_iter()
                        .map(|_| FloatGene::new(min, max).with_bounds(lower_bound, upper_bound))
                        .collect::<Vec<FloatGene>>()))
                .collect::<Vec<Chromosome<FloatGene, f32>>>()
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

/// bit codex
pub fn bit(num_chromosomes: i32, num_genes: i32) -> Codex<BitGene, bool , Vec<Vec<bool>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                        .into_iter()
                        .map(|_| BitGene::new())
                        .collect::<Vec<BitGene>>()))
                .collect::<Vec<Chromosome<BitGene, bool>>>()
        })
        .decoder(|genotype| {
            genotype
                .iter()
                .map(|chromosome| {
                    chromosome
                        .iter()
                        .map(|gene| *gene.allele())
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>()
        })
}


/// int codex
pub fn int(
    num_chromosomes: i32,
    num_genes: i32,
    max: i32,
    min: i32,
) -> Codex<IntGene, i32, Vec<Vec<i32>>> {
    int_with_bounds(num_chromosomes, num_genes, max, min, i32::MAX, i32::MIN)
}

pub fn int_with_bounds(
    num_chromosomes: i32,
    num_genes: i32,
    min: i32,
    max: i32,
    lower_bound: i32,
    upper_bound: i32,
) -> Codex<IntGene, i32, Vec<Vec<i32>>> {
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: (0..num_chromosomes)
                .into_iter()
                .map(|_| Chromosome::from_genes((0..num_genes)
                    .into_iter()
                    .map(|_| IntGene::new(min, max).with_bounds(lower_bound, upper_bound))
                    .collect::<Vec<IntGene>>()))
                .collect::<Vec<Chromosome<IntGene, i32>>>()
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


/// Subset codex
pub fn subset<T: Clone>(alleles: &'static Vec<T>) -> Codex<BitGene, bool, Vec<T>> {
    let count = alleles.len();
    Codex::new()
        .encoder(move || Genotype {
            chromosomes: vec![Chromosome::from_genes((0..count)
                .into_iter()
                .map(|_| BitGene::new())
                .collect::<Vec<BitGene>>())]
        })
        .decoder(|genotype| {
            let chromosome = &genotype.chromosomes[0];
            let mut idx = 0;
            let mut result = Vec::new();
            for gene in chromosome.iter() {
                if *gene.allele() {
                    result.push(alleles[idx].clone());
                }
                idx += 1;
            }

            result
        })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        let codex = char(1, 3);
        let genotype = codex.encode();
        let decoded = codex.decode(&genotype);
        assert_eq!(decoded.len(), 3);
    }

    #[test]
    fn test_float() {
        let codex = float(2, 3, 0.0, 1.0);
        let genotype = codex.encode();
        let decoded = codex.decode(&genotype);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].len(), 3);
    }

    #[test]
    fn test_bit() {
        let codex = bit(2, 3);
        let genotype = codex.encode();
        let decoded = codex.decode(&genotype);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].len(), 3);
    }

    #[test]
    fn test_int() {
        let codex = int(2, 3, 0, 1);
        let genotype = codex.encode();
        let decoded = codex.decode(&genotype);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].len(), 3);
    }
}