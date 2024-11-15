use radiate_rust::engines::alterers::alter::Alterer;
use radiate_rust::engines::codexes::char_codex::CharCodex;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::score::Score;
use radiate_rust::engines::selectors::selector::Selector;

fn main() {
    let target = "Chicago, IL";
    let codex = CharCodex::new(1, target.len());

    let engine =
        GeneticEngine::from_codex(&codex)
            .offspring_selector(Selector::Elitism)
            .survivor_selector(Selector::Tournament(3))
            .alterer(vec![Alterer::Mutator(0.1), Alterer::UniformCrossover(0.5)])
            .fitness_fn(|genotype: String| {
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

    let result = engine.run(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.best);

        output.score().as_usize() == target.len()
    });

    println!("{:?}", result);
}
