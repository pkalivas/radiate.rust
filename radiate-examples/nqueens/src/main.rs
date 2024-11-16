use radiate_rust::*;

const N_QUEENS: usize = 16;

fn main() {
    let codex = IntCodex::new(1, N_QUEENS, 0, N_QUEENS as i32);

    let engine = GeneticEngine::from_codex(&codex)
        .minimizing()
        .offspring_selector(Selector::Tournament(3))
        .alterer(vec![
            Alterer::SinglePointCrossover(0.5),
            Alterer::Mutator(0.01),
        ])
        .fitness_fn(|genotype: Vec<Vec<i32>>| {
            let queens = &genotype[0];
            let mut score = 0;

            for i in 0..N_QUEENS {
                for j in (i + 1)..N_QUEENS {
                    if queens[i] == queens[j] {
                        score += 1;
                    }
                    if (i as i32 - j as i32).abs() == (queens[i] - queens[j]).abs() {
                        score += 1;
                    }
                }
            }

            Score::from_usize(score)
        })
        .build();

    let result = engine.run(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.score().as_usize());

        output.score().as_usize() == 0
    });

    println!("\nResult Queens Board ({:.3?}):", result.timer.elapsed());

    let board = &result.best[0];
    for i in 0..N_QUEENS {
        for j in 0..N_QUEENS {
            if board[j] == i as i32 {
                print!("Q ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
