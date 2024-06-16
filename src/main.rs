mod engines;

use engines::codex::Codex;
use engines::engine::Engine;
use engines::genetic_engine::GeneticEngine;
use engines::genome::genes::gene::Allele;
use engines::genome::genes::int_gene::IntGene;
use engines::genome::genotype::Genotype;
use engines::genome::genes::float_gene::FloatGene;
use engines::genome::chromosome::Chromosome;
use engines::score::Score;
use engines::alterers::mutator::Mutator;
use engines::alterers::uniform_crossover::UniformCrossover;

fn main() {

    let temp = IntGene::new(0, 10);
    let engine = GeneticEngine::builder()
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

    engine.run();
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