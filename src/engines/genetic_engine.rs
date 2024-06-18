use crate::engines::alterers::alter::Alter;
use crate::engines::codex::Codex;
use crate::engines::engine::Engine;
use crate::engines::engine::EngineOutput;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::schema::timer::Timer;
use crate::engines::score::Score;

use super::genome::phenotype::Phenotype;
use super::optimize;
use super::selectors::selector::Select;

pub struct GeneticEngine<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub params: GeneticEngineParams<TGene, T>,
}

impl<TGene, T> GeneticEngine<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub fn new(params: GeneticEngineParams<TGene, T>) -> Self {
        GeneticEngine { params }
    }

    pub fn builder() -> GeneticEngineParams<TGene, T> {
        GeneticEngineParams::new()
    }

    pub fn from_codex(codex: Codex<TGene, T>) -> GeneticEngineParams<TGene, T> {
        GeneticEngineParams::new().codex(codex)
    }

    pub fn evaluate(&self, population: &mut Population<TGene>) {
        let codex = self.codex();
        let fitness_fn = self.fitness_fn();
        let optimize = self.optimize();

        for idx in 0..population.len() {
            let individual = population.get_mut(idx);
            if !individual.score().is_some() {
                let decoded = codex.decode(individual.genotype());
                let score = fitness_fn(&decoded);

                individual.set_score(Some(score));
            }
        }

        optimize.sort(population);
    }

    pub fn select_survivors(&self, population: &Population<TGene>) -> Population<TGene> {
        let selector = self.survivor_selector();
        let count = self.survivor_count();

        selector.select(population, count)
    }

    pub fn select_offspring(&self, population: &Population<TGene>) -> Population<TGene> {
        let selector = self.offspring_selector();
        let count = self.offspring_count();

        selector.select(population, count)
    }

    pub fn alter(&self, population: &mut Population<TGene>, generation: i32) {
        let alterer = self.params.alterer.as_ref().unwrap();
        let optimize = self.optimize();

        alterer.alter(population, optimize, generation);
    }

    pub fn filter(&self, population: &mut Population<TGene>, generation: i32) {
        let max_age = self.params.max_age;
        let codex = self.codex();

        for individual in population.iter_mut() {
            if individual.age(generation) > max_age {
                *individual = Phenotype::from_genotype(codex.encode(), generation);
            }
        }
    }

    pub fn recombine(&self, survivors: Population<TGene>, offspring: Population<TGene>) -> Population<TGene>{
        survivors
            .into_iter()
            .chain(offspring.into_iter())
            .collect::<Population<TGene>>()
    }

    pub fn audit(&self, output: &mut EngineOutput<TGene, T>, population: Population<TGene>) {
        let codex = self.codex();

        let best = codex.decode(&population.get(0).genotype());

        output.population = population;
        output.best = best;
        output.index += 1;
    }

    pub fn survivor_selector(&self) -> &impl Select<TGene> {
        &self.params.survivor_selector
    }

    pub fn offspring_selector(&self) -> &impl Select<TGene> {
        &self.params.offspring_selector
    }

    pub fn codex(&self) -> &Codex<TGene, T> {
        self.params.codex.as_ref().unwrap()
    }

    pub fn fitness_fn(&self) -> &dyn Fn(&T) -> Score {
        self.params.fitness_fn.as_ref().unwrap()
    }

    pub fn population(&self) -> &Population<TGene> {
        self.params.population.as_ref().unwrap()
    }

    pub fn optimize(&self) -> &Optimize {
        &self.params.optimize
    }

    pub fn survivor_count(&self) -> usize {
        self.params.population_size - self.offspring_count()
    }

    pub fn offspring_count(&self) -> usize {
        (self.params.population_size as f32 * self.params.offspring_fraction) as usize
    }
}

impl<TGene, T> Engine<TGene, T> for GeneticEngine<TGene, T>
where
    TGene: Gene<TGene>,
    T: Clone,
{
    fn start(&self) -> EngineOutput<TGene, T> {
        let population = self.population();

        EngineOutput {
            population: population.clone(),
            best: self.codex().decode(&population.get(0).genotype()),
            index: 0,
            timer: Timer::new(),
        }
    }

    fn fit<F>(&self, limit: F) -> EngineOutput<TGene, T>
    where
        F: Fn(&EngineOutput<TGene, T>) -> bool,
    {
        let mut output = self.start();

        loop {
            self.evaluate(&mut output.population);

            let mut survivors = self.select_survivors(&output.population);
            let mut offspring = self.select_offspring(&output.population);

            self.alter(&mut offspring, output.index);

            self.filter(&mut survivors, output.index);
            self.filter(&mut offspring, output.index);

            let mut new_population = self.recombine(survivors, offspring);

            self.evaluate(&mut new_population);
            self.audit(&mut output, new_population);

            if limit(&output) {
                break;
            }
        }

        self.stop(&mut output)
    }
}
