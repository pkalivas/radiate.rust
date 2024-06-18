mod engines;

use crate::engines::selectors::selector::Selector;
use engines::alterers::alter::{Alter, Alterer};
use engines::codex::Codex;
use engines::engine::Engine;
use engines::genetic_engine::GeneticEngine;
use engines::genome::chromosome::Chromosome;
use engines::genome::genes::char_gene::CharGene;
use engines::genome::genes::float_gene::FloatGene;
use engines::genome::genes::gene::Allele;
use engines::genome::genes::int_gene::IntGene;
use engines::genome::genotype::Genotype;
use engines::score::Score;

fn main() {
    // run_min_sum();
    run_string_evolve("Chicago, IL");
}

fn run_string_evolve(target: &'static str) {
    let codex = get_char_codex(1, target.len());

    let engine = GeneticEngine::from_codex(codex)
            .offspring_selector(Selector::Tournament(3))
            .survivor_selector(Selector::Elitism)
            .alterers(vec![
                Alterer::Mutator(0.001),
                Alterer::UniformCrossover(0.5),
            ])
            .fitness_fn(|genotype: &String| {
                Score::from_usize(genotype.chars().zip(target.chars()).fold(
                    0,
                    |acc, (geno, targ)| {
                        if geno == targ {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                ))
            })
            .build();

    let result = engine.fit(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.best);
        output.score() == target.len() as f32
    });

    println!("{:?}", result);
}

fn run_min_sum() {
    let codex = get_int_codex(1, 10, 0, 100);

    let engine = GeneticEngine::from_codex(codex)
        .population_size(100)
        .minimizing()
        .offspring_selector(Selector::Elitism)
        .survivor_selector(Selector::Tournament(4))
        .alterers(vec![
            Alterer::SwapMutator(0.001),
            Alterer::Mutator(1e-5),
            Alterer::UniformCrossover(0.5),
        ])
        .fitness_fn(|genotype: &Vec<Vec<i32>>| {
            Score::from_int(genotype.iter().fold(0, |acc, chromosome| {
                acc + chromosome.iter().fold(0, |acc, gene| acc + gene)
            }))
        })
        .build();

    let result = engine.fit(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.best);
        output.score() == 0.0
    });

    println!("{:?}", result);
}

fn get_char_codex(num_chromosomes: usize, num_genes: usize) -> Codex<CharGene, String> {
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

fn get_float_codex(
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

fn get_int_codex(
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
