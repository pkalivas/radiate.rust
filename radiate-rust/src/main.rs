mod engines;

use engines::alterers::alter::Alterer;
use engines::alterers::mutators::numeric_mutator::NumericMutator;
use engines::codex;
use engines::engine::Engine;
use engines::genetic_engine::GeneticEngine;
use engines::score::Score;
use engines::selectors::selector::Selector;

fn main() {
    let options = String::from(
        "0. Exit\n\
        1. Minimize sum\n\
        2. Evolve string\n"
    );

    loop {
        print!("{}", options);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => run_min_sum(),
            "2" => run_string_evolve("Chicago, IL"),
            "0" => return,
            _ => println!("Invalid option"),
        }
    }
}

fn run_string_evolve(target: &'static str) {
    let codex = codex::char(1, target.len());

    let engine = GeneticEngine::from_codex(codex)
        .offspring_selector(Selector::Elitism)
        .survivor_selector(Selector::Tournament(3))
        .alterer(vec![
            Alterer::Mutator(0.01),
            Alterer::UniformCrossover(0.5)
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
    let codex = codex::int_with_bounds(1, 10, 0, 100, 0, 100);

    let engine = GeneticEngine::from_codex(codex)
        .population_size(100)
        .minimizing()
        .offspring_selector(Selector::Elitism)
        .survivor_selector(Selector::Tournament(4))
        .alterer(vec![
            Alterer::Mutation(Box::new(NumericMutator::new(0.001))),
            Alterer::UniformCrossover(0.5)
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