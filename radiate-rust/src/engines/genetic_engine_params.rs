use std::sync::Arc;

use crate::engines::alterers::composite_alterer::CompositeAlterer;
use crate::engines::codex::Codex;
use crate::engines::genetic_engine::GeneticEngine;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::problem::{DefaultProblem, Problem};
use crate::engines::score::Score;
use crate::engines::selectors::selector::Selector;

use super::alterers::alter::Alterer;

pub struct GeneticEngineParams<'a, G, A, T>
where
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a
{
    pub population_size: usize,
    pub max_age: i32,
    pub offspring_fraction: f32,
    pub optimize: Optimize,
    pub survivor_selector: Selector,
    pub offspring_selector: Selector,
    pub alterer: Option<CompositeAlterer<G, A>>,
    pub codex: Option<Codex<G, A, T>>,
    pub population: Option<Population<G, A>>,
    pub fitness_fn: Option<Arc<dyn Fn(&T) -> Score>>,
    pub problem: Option<Arc<dyn Problem<'a, G, A, T>>>,
}

impl<'a, G, A, T> GeneticEngineParams<'a, G, A, T> 
where
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a,
    'a: 'static
{
    pub fn new() -> Self {
        GeneticEngineParams {
            population_size: 100,
            max_age: 25,
            offspring_fraction: 0.8,
            optimize: Optimize::Maximize,
            survivor_selector: Selector::Tournament(3),
            offspring_selector: Selector::Roulette,
            alterer: None,
            codex: None,
            population: None,
            fitness_fn: None,
            problem: None//Some(Box::new(DefaultProblem::new())),
        }
    }

    pub fn population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    pub fn max_age(mut self, max_age: i32) -> Self {
        self.max_age = max_age;
        self
    }

    pub fn offspring_fraction(mut self, offspring_fraction: f32) -> Self {
        self.offspring_fraction = offspring_fraction;
        self
    }

    pub fn codex(mut self, codex: Codex<G, A, T>) -> Self {
        self.codex = Some(codex);
        self
    }

    pub fn population(mut self, population: Population<G, A>) -> Self {
        self.population = Some(population);
        self
    }

    pub fn fitness_fn(mut self, fitness_func: impl Fn(&T) -> Score + 'static) -> Self {
        self.fitness_fn = Some(Arc::new(fitness_func));
        self
    }

    pub fn survivor_selector(mut self, selector: Selector) -> Self {
        self.survivor_selector = selector;
        self
    }

    pub fn offspring_selector(mut self, selector: Selector) -> Self {
        self.offspring_selector = selector;
        self
    }

    pub fn alterer(mut self, alterers: Vec<Alterer<G, A>>) -> Self {
        self.alterer = Some(CompositeAlterer::new(alterers));
        self
    }

    pub fn minimizing(mut self) -> Self {
        self.optimize = Optimize::Minimize;
        self
    }

    pub fn maximizing(mut self) -> Self {
        self.optimize = Optimize::Maximize;
        self
    }

    pub fn problem(mut self, problem: impl Problem<'a, G, A, T> + 'static) -> Self {
        self.problem = Some(Arc::new(problem));
        self
    }

    pub fn build(mut self) -> GeneticEngine<'a, G, A, T> {
        self.build_population();
        self.build_alterer();
        self.build_problem();

        GeneticEngine::new(self)
    }

    fn build_population(&mut self) {
        self.population = match &self.population {
            None => Some(match self.codex.as_ref() {
                Some(codex) => Population::from_func(self.population_size, || {
                    Phenotype::from_genotype(codex.encode(), 0)
                }),
                None => panic!("Codex not set"),
            }),
            Some(pop) => Some(pop.clone()),
        };
    }

    fn build_alterer(&mut self) {
        if self.alterer.is_none() {
            self.alterer = Some(CompositeAlterer::new(vec![
                Alterer::Mutator(0.001),
                Alterer::UniformCrossover(0.5),
            ]));
        }
    }

    fn build_problem(&mut self) {
        if self.problem.is_some() {
            return;
        }

        if self.fitness_fn.is_none() {
            panic!("Fitness function not set");
        }

        if self.codex.is_none() {
            panic!("Codex not set");
        }

        let encoder = self.codex.as_ref().unwrap().encoder.clone().unwrap();
        let decoder = self.codex.as_ref().unwrap().decoder.clone().unwrap();

        self.problem = Some(Arc::new(DefaultProblem::new()
            .encoder(encoder)
            .decoder(decoder)
            .fitness_fn(self.fitness_fn.as_ref().unwrap().clone())
        ));
    }
}
