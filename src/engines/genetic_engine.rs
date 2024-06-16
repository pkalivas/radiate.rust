use crate::engines::genome::genes::gene::Gene;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::engine::Engine;

pub struct GeneticEngine<TGene, T> 
    where TGene : Gene<TGene>
{
    pub params: GeneticEngineParams<TGene, T>,
}

impl<TGene, T> GeneticEngine<TGene, T>
    where TGene : Gene<TGene>
{
    pub fn new(params: GeneticEngineParams<TGene, T>) -> Self {
        GeneticEngine { params }
    }

    pub fn offspring_count(&self) -> usize {
        (self.params.population_size as f32 * self.params.offspring_fraction) as usize
    }

    pub fn surivor_count(&self) -> usize {
        self.params.population_size as usize - self.offspring_count()
    }
}

impl<TGene, T> Engine for GeneticEngine<TGene, T>
    where TGene : Gene<TGene>
{
    fn run(&self) {

        let codex = self.params.codex.as_ref().unwrap();
        let fitness_fn = self.params.fitness_fn.as_ref().unwrap();
        let mut population = self.params.population.as_ref().unwrap().clone();
        
        for i in 0..100 {
            println!("Generation: {}", i);

            for idx in 0..population.len() {
                let individual = population.get_mut(idx).unwrap();
                if !individual.score.is_some() {
                    let decoded = codex.decode(&individual.genotype);
                    let score = fitness_fn(&decoded);

                    individual.score = Some(score);
                }
            }

            population.sort();

            let survivors = population.individuals.iter()
                .take(self.surivor_count())
                .map(|individual| individual.clone())
                .collect::<Vec<_>>();

            let offspring = population.individuals.iter()
                .take(self.offspring_count())
                .map(|individual| individual.clone())
                .collect::<Vec<_>>();

            let t = "";

        }
    }
}