use crate::engines::alterers::alter::Alter;
use crate::engines::codex::Codex;
use crate::engines::engine::Engine;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::schema::timer::Timer;
use crate::engines::score::Score;

use super::engine_context::EngineContext;
use super::genome::phenotype::Phenotype;
use super::selectors::selector::Select;

pub struct GeneticEngine<G, A, T>
where
    G: Gene<G, A>
{
    pub params: GeneticEngineParams<G, A, T>,
}

impl<G, A, T> GeneticEngine<G, A, T>
where
    G: Gene<G, A>
{

    pub fn new(params: GeneticEngineParams<G, A, T>) -> Self {
        GeneticEngine { params }
    }

    pub fn from_codex(codex: Codex<G, A, T>) -> GeneticEngineParams<G, A, T> {
        GeneticEngineParams::new().codex(codex)
    }

    pub fn evaluate(&self, handle: &mut EngineContext<G, A, T>) {
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

    pub fn select_survivors(&self, population: &Population<G, A>) -> Population<G, A> {
        let selector = self.survivor_selector();
        let count = self.survivor_count();

        selector.select(population, count)
    }

    pub fn select_offspring(&self, population: &Population<G, A>) -> Population<G, A> {
        let selector = self.offspring_selector();
        let count = self.offspring_count();

        selector.select(population, count)
    }

    pub fn alter(&self, population: &mut Population<G, A>, generation: i32) {
        let alterer = self.alterer();
        let optimize = self.optimize();

        alterer.alter(population, optimize, generation);
    }

    pub fn filter(&self, population: &mut Population<G, A>, generation: i32) {
        let max_age = self.params.max_age;
        let codex = self.codex();

        for individual in population.iter_mut() {
            if individual.age(generation) > max_age {
                *individual = Phenotype::from_genotype(codex.encode(), generation);
            } else if !individual.genotype().is_valid() {
                *individual = Phenotype::from_genotype(codex.encode(), generation);
            }
        }
    }

    pub fn recombine(
        &self,
        handle: &mut EngineContext<G, A, T>,
        survivors: Population<G, A>,
        offspring: Population<G, A>,
    ) {
        handle.population = survivors
            .into_iter()
            .chain(offspring.into_iter())
            .collect::<Population<G, A>>();
    }

    pub fn audit(&self, output: &mut EngineContext<G, A, T>) {
        let codex = self.codex();

        if !output.population.is_sorted {
            self.optimize().sort(&mut output.population);
        }
        
        output.best = codex.decode(&output.population.get(0).genotype());
        output.index += 1;
    }

    pub fn survivor_selector(&self) -> &impl Select<G, A> {
        &self.params.survivor_selector
    }

    pub fn offspring_selector(&self) -> &impl Select<G, A> {
        &self.params.offspring_selector
    }

    pub fn alterer(&self) -> &impl Alter<G, A> {
        self.params.alterer.as_ref().unwrap()
    }

    pub fn codex(&self) -> &Codex<G, A, T> {
        self.params.codex.as_ref().unwrap()
    }

    pub fn fitness_fn(&self) -> &impl Fn(&T) -> Score {
        self.params.fitness_fn.as_ref().unwrap()
    }

    pub fn population(&self) -> &Population<G, A> {
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

impl<G: Gene<G, A>, A, T: Clone> Engine<G, A, T> for GeneticEngine<G, A, T> {
    fn fit<F: Fn(&EngineContext<G, A, T>) -> bool>(&self, limit: F) -> EngineContext<G, A, T> {
        let mut ctx = self.start();

        loop {
            self.evaluate(&mut ctx);

            let mut survivors = self.select_survivors(&ctx.population);
            let mut offspring = self.select_offspring(&ctx.population);

            self.alter(&mut offspring, ctx.index);

            self.filter(&mut survivors, ctx.index);
            self.filter(&mut offspring, ctx.index);

            self.recombine(&mut ctx, survivors, offspring);

            self.evaluate(&mut ctx);
            self.audit(&mut ctx);

            if limit(&ctx) {
                break self.stop(&mut ctx)
            }
        }
    }

    fn start(&self) -> EngineContext<G, A, T> {
        let population = self.population();

        EngineContext {
            population: population.clone(),
            best: self.codex().decode(&population.get(0).genotype()),
            index: 0,
            timer: Timer::new(),
        }
    }
}
