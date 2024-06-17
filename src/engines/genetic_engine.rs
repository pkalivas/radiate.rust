use crate::engines::alterers::alter::Alter;
use crate::engines::codex::Codex;
use crate::engines::engine::Engine;
use crate::engines::engine::EngineOutput;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::score::Score;

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

    pub fn evaluate(&self, population: &mut Population<TGene>) {
        let codex = self.codex();
        let fitness_fn = self.fitness_fn();

        for idx in 0..population.len() {
            let individual = population.get_mut(idx);
            if !individual.score().is_some() {
                let decoded = codex.decode(individual.genotype());
                let score = fitness_fn(&decoded);

                individual.set_score(score);
            }
        }

        population.sort();
    }

    pub fn select_survivors(&self, population: &Population<TGene>) -> Population<TGene> {
        population
            .iter()
            .take(self.surivor_count())
            .map(|individual| individual.clone())
            .collect::<Population<TGene>>()
    }

    pub fn select_offspring(&self, population: &Population<TGene>) -> Population<TGene> {
        population
            .iter()
            .take(self.offspring_count())
            .map(|individual| individual.clone())
            .collect::<Population<TGene>>()
    }

    pub fn alter(&self, population: &mut Population<TGene>) {
        let alterer = self.params.alterer.as_ref().unwrap();
        alterer.alter(population);
    }

    pub fn audit(&self, output: &mut EngineOutput<TGene, T>) {
        output.population.sort();
        output.index += 1;
        output.best = self.codex().decode(&output.population.get(0).genotype());
    }

    pub fn offspring_count(&self) -> usize {
        (self.params.population_size as f32 * self.params.offspring_fraction) as usize
    }

    pub fn surivor_count(&self) -> usize {
        self.params.population_size as usize - self.offspring_count()
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
}

impl<TGene, T> Engine<TGene, T> for GeneticEngine<TGene, T>
where
    TGene: Gene<TGene>,
{
    fn fit<F>(&self, limit: F) -> EngineOutput<TGene, T>
    where 
        F: Fn(&EngineOutput<TGene, T>) -> bool 
    {
        let mut output = EngineOutput {
            population: self.population().clone(),
            best: self.codex().decode(&self.population().get(0).genotype()),
            index: 0,
        };

        loop {
            self.evaluate(&mut output.population);

            let suvivors = self.select_survivors(&output.population);

            let mut offspring = self.select_offspring(&output.population);

            self.alter(&mut offspring);
            self.evaluate(&mut offspring);

            output.population.replace(suvivors.into_iter().chain(offspring.into_iter()).collect());
            
            self.audit(&mut output);

            if limit(&output) {
                break;
            }
        }

        output
    }
}
