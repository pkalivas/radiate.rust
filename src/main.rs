mod engines;

use engines::codex::Codex;
use engines::engine::Engine;
use engines::genetic_engine_params::GeneticEngineParams;
use engines::genome::genes::gene::Allele;
use engines::genome::genotype::Genotype;
use engines::genome::genes::float_gene::FloatGene;
use engines::genome::chromosome::Chromosome;
use engines::score::Score;

fn main() {
 
    let engine = GeneticEngineParams::new()
        .codex(get_float_codex(1, 10))
        .fitness_func(|genotype: &Vec<Vec<f32>>| {
            let mut sum = 0.0;
            for chromosome in genotype {
                for gene in chromosome {
                    sum += gene;
                }
            }

            Score::from_float(sum)
        })
        .build();

    engine.run();
}

fn get_float_codex(num_chromosomes: i32, num_genes: i32) -> Codex<FloatGene, Vec<Vec<f32>>> {
    Codex::new()
        .encoder(move || {
            Genotype { 
                chromosomes: (0..num_chromosomes)
                    .into_iter()
                    .map(|_| {
                        Chromosome {
                            genes: (0..num_genes)
                                .into_iter()
                                .map(|_| FloatGene::new(0_f32, 50_f32))
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