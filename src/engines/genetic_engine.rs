use crate::engines::alterers::alter::Alter;
use crate::engines::codex::Codex;
use crate::engines::engine::Engine;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::schema::timer::Timer;
use crate::engines::score::Score;

use super::engine_handle::EngineHandle;
use super::genome::phenotype::Phenotype;
use super::selectors::selector::Select;

pub struct GeneticEngine<TGene: Gene<TGene>, T> {
    pub params: GeneticEngineParams<TGene, T>,
}

impl<TGene: Gene<TGene>, T> GeneticEngine<TGene, T> {
    
    pub fn new(params: GeneticEngineParams<TGene, T>) -> Self {
        GeneticEngine { params }
    }

    pub fn builder() -> GeneticEngineParams<TGene, T> {
        GeneticEngineParams::new()
    }

    pub fn from_codex(codex: Codex<TGene, T>) -> GeneticEngineParams<TGene, T> {
        GeneticEngineParams::new().codex(codex)
    }

    pub fn evaluate(&self, handle: &mut EngineHandle<TGene, T>) {
        let codex = self.codex();
        let fitness_fn = self.fitness_fn();
        let optimize = self.optimize();

        for idx in 0..handle.population.len() {
            let individual = handle.population.get_mut(idx);
            if !individual.score().is_some() {
                let decoded = codex.decode(individual.genotype());
                let score = fitness_fn(&decoded);

                individual.set_score(Some(score));
            }
        }

        optimize.sort(&mut handle.population);
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

    pub fn recombine(&self, handle: &mut EngineHandle<TGene, T>, survivors: Population<TGene>, offspring: Population<TGene>) {
        handle.population = survivors
            .into_iter()
            .chain(offspring.into_iter())
            .collect::<Population<TGene>>();
    }

    pub fn audit(&self, output: &mut EngineHandle<TGene, T>) {
        let codex = self.codex();

        let best = codex.decode(&output.population.get(0).genotype());

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

impl<TGene: Gene<TGene>, T: Clone> Engine<TGene, T> for GeneticEngine<TGene, T> {
    fn fit<F: Fn(&EngineHandle<TGene, T>) -> bool>(&self, limit: F) -> EngineHandle<TGene, T> {
        let mut handle = self.start();

        loop {
            self.evaluate(&mut handle);

            let mut survivors = self.select_survivors(&handle.population);
            let mut offspring = self.select_offspring(&handle.population);

            self.alter(&mut offspring, handle.index);

            self.filter(&mut survivors, handle.index);
            self.filter(&mut offspring, handle.index);

            self.recombine(&mut handle, survivors, offspring);

            self.evaluate(&mut handle);
            self.audit(&mut handle);

            if limit(&handle) {
                break;
            }
        }

        self.stop(&mut handle)
    }

    fn start(&self) -> EngineHandle<TGene, T> {
        let population = self.population();

        EngineHandle {
            population: population.clone(),
            best: self.codex().decode(&population.get(0).genotype()),
            index: 0,
            timer: Timer::new(),
        }
    }
}
