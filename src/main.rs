mod engines;

use engines::codex::Codex;
use engines::engine::{Engine, EngineOutput};
use engines::genetic_engine::GeneticEngine;
use engines::genome::genes::char_gene::CharGene;
use engines::genome::genes::gene::Allele;
use engines::genome::genes::int_gene::IntGene;
use engines::genome::genotype::Genotype;
use engines::genome::genes::float_gene::FloatGene;
use engines::genome::chromosome::Chromosome;
use engines::genome::population::Population;
use engines::score::Score;
use engines::alterers::mutator::Mutator;
use engines::alterers::uniform_crossover::UniformCrossover;

fn main() {
    // run_min_sum();
    run_string_evolve("Chicago, IL");
}

fn run_string_evolve(target: &'static str) {
    let codex = get_char_codex(1, target.len());
    let codex_2 = get_char_codex(1, target.len());

    let engine = GeneticEngine::builder()
        .population_size(1000)
        .offspring_fraction(0.8)
        .codex(codex)
        .alterers(vec![
            Box::new(UniformCrossover::new(0.5)),
            Box::new(Mutator::new(0.001)),
        ])
        .fitness_fn(|genotype: &String| Score::from_usize(genotype.chars()
            .zip(target.chars())
            .fold(target.len(), |acc, (geno, targ)| {
                if geno == targ {
                    acc - 1
                } else {
                    acc
                }
            })))
        .build();

    let result = engine.fit();

    println!("{:?}", result.best);

    // let result: EngineOutput<CharGene, String> = engine.into_iter()
    //     .take(1000)
    //     .min_by(|a, b| b.score().partial_cmp(&a.score()).unwrap())
    //     .unwrap();

    // println!("{:?}", result.best);

    // engine.run(|pop: &Population<CharGene>| {
    //     let best = pop.get(0).genotype();
    //     let best_str = codex_2.decode(best);
    //     println!("{}", best_str);
    //     best_str == target
    // });
}

fn run_min_sum() {
    let engine = GeneticEngine::builder()
        .population_size(1000)
        .offspring_fraction(0.8)
        .codex(get_int_codex(1, 10, 0, 100))
        .alterers(vec![
            Box::new(UniformCrossover::new(0.5)),
            Box::new(Mutator::new(0.001)),
        ])
        .fitness_fn(|genotype: &Vec<Vec<i32>>| {
            let mut sum = 0;
            for chromosome in genotype {
                for gene in chromosome {
                    sum += gene;
                }
            }

            Score::from_int(sum)
        })
        .build();

    // engine.run(|pop: &Population<IntGene>| {
    //     let best = pop.get(0).genotype();
    //     let best_sum = best.iter().fold(0, |acc, chromosome| {
    //         acc + chromosome.iter().fold(0, |acc, gene| acc + gene.allele())
    //     });
    //     println!("Best: {}", best_sum);
    //     best_sum != 0
    // });

}

fn get_char_codex(num_chromosomes: usize, num_genes: usize) -> Codex<CharGene, String> {
    Codex::new()
        .encoder(move || {
            Genotype { 
                chromosomes: (0..num_chromosomes)
                    .into_iter()
                    .map(|_| {
                        Chromosome {
                            genes: (0..num_genes)
                                .into_iter()
                                .map(|_| CharGene::new())
                                .collect::<Vec<CharGene>>()
                        }
                    })
                    .collect::<Vec<Chromosome<CharGene>>>()
            }
        })
        .decoder(|genotype| {
            genotype.chromosomes.iter().map(|chromosome| {
                chromosome.genes.iter().map(|gene| {
                    *gene.allele() as u8 as char
                }).collect::<String>()
            }).collect::<String>()
        })
}

fn get_float_codex(num_chromosomes: i32, num_genes: i32, min: f32, max: f32) -> Codex<FloatGene, Vec<Vec<f32>>> {
    Codex::new()
        .encoder(move || {
            Genotype { 
                chromosomes: (0..num_chromosomes)
                    .into_iter()
                    .map(|_| {
                        Chromosome {
                            genes: (0..num_genes)
                                .into_iter()
                                .map(|_| FloatGene::new(min, max))
                                .collect::<Vec<FloatGene>>()
                        }
                    })
                    .collect::<Vec<Chromosome<FloatGene>>>()
            }
        })
        .decoder(|genotype| {
            genotype.chromosomes.iter().map(|chromosome| {
                chromosome.genes.iter().map(|gene| {
                    *gene.allele()
                }).collect::<Vec<f32>>()
            }).collect::<Vec<Vec<f32>>>()
        })
}

fn get_int_codex(num_chromosomes: i32, num_genes: i32, max: i32, min: i32) -> Codex<IntGene, Vec<Vec<i32>>> {
    Codex::new()
        .encoder(move || {
            Genotype { 
                chromosomes: (0..num_chromosomes)
                    .into_iter()
                    .map(|_| {
                        Chromosome {
                            genes: (0..num_genes)
                                .into_iter()
                                .map(|_| IntGene::new(min, max))
                                .collect::<Vec<IntGene>>()
                        }
                    })
                    .collect::<Vec<Chromosome<IntGene>>>()
            }
        })
        .decoder(|genotype| {
            genotype.chromosomes.iter().map(|chromosome| {
                chromosome.genes.iter().map(|gene| {
                    *gene.allele()
                }).collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>()
        })
}