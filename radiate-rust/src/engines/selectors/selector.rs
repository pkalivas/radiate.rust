use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use rand::Rng;

pub trait Select<G, A> 
where
    G: Gene<G, A>
{
    fn select(&self, population: &Population<G, A>, count: usize) -> Population<G, A>;
}

#[allow(dead_code)]
pub enum Selector {
    Tournament(u8),
    Roulette,
    Rank,
    Elitism,
    Boltzmann(f32),
}

impl Selector {
    pub fn total_fitness<G, A>(&self, population: &Population<G, A>) -> f32
    where
        G: Gene<G, A>
    {
        population
            .iter()
            .map(|i| match i.score() {
                Some(score) => score.as_float(),
                None => 0.0,
            })
            .sum::<f32>()
    }
}

impl<G, A> Select<G, A> for Selector 
where
    G: Gene<G, A>
{
    fn select(&self, population: &Population<G, A>, count: usize) -> Population<G, A> {
        match self {
            Selector::Tournament(size) => {
                let mut rng = rand::thread_rng();
                let mut selected = Vec::with_capacity(count);

                for _ in 0..count {
                    let mut tournament = Vec::with_capacity(*size as usize);
                    for _ in 0..*size {
                        let idx = rng.gen_range(0..population.len());
                        tournament.push(idx);
                    }

                    tournament.sort();

                    selected.push(population.get(tournament[0]).clone());
                }

                Population::from_vec(selected)
            }
            Selector::Roulette => {
                let mut selected = Vec::with_capacity(count);
                let mut rng = rand::thread_rng();
                let total_fitness = self.total_fitness(population);

                for _ in 0..count {
                    let mut idx = rng.gen_range(0.0..total_fitness);

                    for individual in population.iter() {
                        idx -= match individual.score() {
                            Some(score) => score.as_float(),
                            None => 0.0,
                        };

                        if idx <= 0.0 {
                            selected.push(individual.clone());
                            break;
                        }
                    }
                }

                Population::from_vec(selected)
            }
            Selector::Rank => {
                let mut selected = Vec::with_capacity(count);
                let mut rng = rand::thread_rng();

                let total_rank = (population.len() * (population.len() + 1)) as f32 / 2.0;

                for _ in 0..count {
                    let mut idx = rng.gen_range(0.0..total_rank);
                    let mut selected_idx = 0;
                    for individual in population.iter() {
                        idx -= (population.len() - selected_idx) as f32;
                        if idx <= 0.0 {
                            selected.push(individual.clone());
                            break;
                        }
                        selected_idx += 1;
                    }
                }

                Population::from_vec(selected)
            }
            Selector::Elitism => population
                .iter()
                .take(count)
                .map(|individual| individual.clone())
                .collect::<Population<G, A>>(),
            Selector::Boltzmann(temperature) => {
                let mut selected = Vec::with_capacity(count);
                let mut rng = rand::thread_rng();

                let total_fitness = self.total_fitness(population);

                for _ in 0..count {
                    let mut idx = rng.gen_range(0.0..total_fitness);
                    for individual in population.iter() {
                        let fitness = match individual.score() {
                            Some(score) => score.as_float(),
                            None => 0.0,
                        };
                        let probability = (fitness / temperature).exp();
                        idx -= probability;
                        if idx <= 0.0 {
                            selected.push(individual.clone());
                            break;
                        }
                    }
                }

                Population::from_vec(selected)
            }
        }
    }
}
