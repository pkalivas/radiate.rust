use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::Optimize;
use rand::Rng;

pub trait Select<G, A> 
where
    G: Gene<G, A>
{
    fn select(&self, population: &Population<G, A>, optimize: &Optimize, count: usize) -> Population<G, A>;
}

pub enum Selector {
    Tournament(u8),
    Roulette,
    Rank,
    Elitism,
    Boltzmann(f32),
}

impl<G, A> Select<G, A> for Selector 
where
    G: Gene<G, A>
{
    #[inline]
    fn select(&self, population: &Population<G, A>, optimize: &Optimize, count: usize) -> Population<G, A> {
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
                let mut fitness_values = Vec::with_capacity(population.len());
                let mut rng = rand::thread_rng();
                
                let total = population
                    .iter()
                    .map(|individual| match individual.score() {
                        Some(score) => score.as_float(),
                        None => 0.0,
                    })
                    .sum::<f32>();

                for individual in population.iter() {
                    let score = match individual.score() {
                        Some(score) => score.as_float(),
                        None => 0.0,
                    };

                    fitness_values.push(score / total);
                }

                if optimize == &Optimize::Minimize {
                    fitness_values.reverse();
                }

                let total_fitness = fitness_values.iter().sum::<f32>();

                for _ in 0..count {
                    let mut idx = rng.gen_range(0.0..total_fitness);

                    for i in 0..fitness_values.len() {
                        idx -= fitness_values[i];
                        if idx <= 0.0 {
                            selected.push(population.get(i).clone());
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

                let mut min = population.get(0).score().as_ref().unwrap().as_float();
                let mut max = min;

                for individual in population.iter() {
                    let score = individual.score().as_ref().unwrap().as_float();
                    if score < min {
                        min = score;
                    }
                    if score > max {
                        max = score;
                    }
                }

                let diff = max - min;
                if diff == 0.0 {
                    return population
                        .iter()
                        .take(count)
                        .map(|individual| individual.clone())
                        .collect::<Population<G, A>>();
                }

                let mut result = Vec::with_capacity(population.len());
                for individual in population.iter() {
                    let score = individual.score().as_ref().unwrap().as_float();
                    let fitness = (score - min) / diff;
                    let value = (temperature * fitness).exp();

                    result.push(value);
                }

                let total_fitness = result.iter().sum::<f32>();
                for i in 0..result.len() {
                    result[i] /= total_fitness;
                }

                if optimize == &Optimize::Minimize {
                    result.reverse();
                }

                let total_fitness = result.iter().sum::<f32>();

                for _ in 0..count {
                    let mut idx = rng.gen_range(0.0..total_fitness);

                    for i in 0..result.len() {
                        idx -= result[i];
                        if idx <= 0.0 {
                            selected.push(population.get(i).clone());
                            break;
                        }
                    }
                }

                Population::from_vec(selected)
            }
        }
    }
}
