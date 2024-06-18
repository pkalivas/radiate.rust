use crate::engines::alterers::alter::{Alter, Alterer};
use crate::engines::alterers::composite_alterer::CompositeAlterer;
use crate::engines::alterers::crossovers::crossover::Crossover;
use crate::engines::codex::Codex;
use crate::engines::genetic_engine::GeneticEngine;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;
use crate::engines::genome::population::Population;
use crate::engines::optimize::Optimize;
use crate::engines::score::Score;
use crate::engines::selectors::selector::Selector;

pub struct GeneticEngineParams<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub population_size: usize,
    pub max_age: i32,
    pub offspring_fraction: f32,
    pub optimize: Optimize,
    pub survivor_selector: Selector,
    pub offspring_selector: Selector,
    pub alterer: Option<CompositeAlterer<TGene>>,
    pub codex: Option<Codex<TGene, T>>,
    pub population: Option<Population<TGene>>,
    pub fitness_fn: Option<Box<dyn Fn(&T) -> Score>>,
}

impl<TGene: Gene<TGene>, T> GeneticEngineParams<TGene, T> {
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

    pub fn codex(mut self, codex: Codex<TGene, T>) -> Self {
        self.codex = Some(codex);
        self
    }

    pub fn population(mut self, population: Population<TGene>) -> Self {
        self.population = Some(population);
        self
    }

    pub fn fitness_fn(mut self, fitness_func: impl Fn(&T) -> Score + 'static) -> Self {
        self.fitness_fn = Some(Box::new(fitness_func));
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

    pub fn temp(mut self, alters: Vec<impl Crossover<TGene>>) -> Self {
        // self.alterer = Some(CompositeAlterer::new(alters));
        self
    }

    pub fn alterers(mut self, alters: Vec<Alterer>) -> Self {
        self.alterer = Some(CompositeAlterer::new(alters));
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

    pub fn build(mut self) -> GeneticEngine<TGene, T> {
        self.build_population();

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
}
