use radiate_rust::engines::alterers::alter::Alterer;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::codex;
use radiate_rust::engines::score::Score;
use radiate_rust::engines::selectors::selector::Selector;
use radiate_rust::engines::engine::Engine;

fn main() {
    let target = "Chicago, IL";
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

    println!("{:?}", result);}
