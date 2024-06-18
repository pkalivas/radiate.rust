mod engines;

use crate::engines::selectors::selector::Selector;
use engines::alterers::alter::{Alter, Alterer};
use engines::engine::Engine;
use engines::genetic_engine::GeneticEngine;
use engines::genome::genes::gene::Allele;
use engines::score::Score;
use crate::engines::codex;

fn main() {
    let mut input = String::from("");

    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);
    // run_min_sum();
    // run_string_evolve("Chicago, IL");
}

fn run_string_evolve(target: &'static str) {
    let codex = codex::char(1, target.len());

    let engine =
        GeneticEngine::from_codex(codex)
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
    let codex = codex::int(1, 10, 0, 100);

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
