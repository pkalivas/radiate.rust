use crate::engines::genome::genes::gene::Gene;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::engine::Engine;
use crate::engines::genome::population::Population;

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

    pub fn builder() -> GeneticEngineParams<TGene, T> {
        GeneticEngineParams::new()
    }

    pub fn offspring_count(&self) -> usize {
        (self.params.population_size as f32 * self.params.offspring_fraction) as usize
    }

    pub fn surivor_count(&self) -> usize {
        self.params.population_size as usize - self.offspring_count()
    }
}

impl<TGene, T> Engine for GeneticEngine<TGene, T>
    where TGene : Gene<TGene>,
         T : std::fmt::Debug
{
    fn run(&self) {

        let codex = self.params.codex.as_ref().unwrap();
        let fitness_fn = self.params.fitness_fn.as_ref().unwrap();
        let alterers = self.params.alterers.as_ref().unwrap();
        let mut population = self.params.population.as_ref().unwrap().clone();
        
        for i in 0..500 {

            for idx in 0..population.len() {
                let individual = population.get_mut(idx).unwrap();
                if !individual.score.is_some() {
                    let decoded = codex.decode(&individual.genotype);
                    let score = fitness_fn(&decoded);

                    individual.score = Some(score);
                }
            }

            population.sort();

            let best_phenotype = population.get(0).unwrap();
            let decoded = codex.decode(&best_phenotype.genotype);

            println!("Best Phenotype: {:?}", decoded);

            let survivors = population.iter()
                .take(self.surivor_count())
                .map(|individual| individual.clone())
                .collect::<Population<TGene>>();

            let mut offspring = population.iter()
                .take(self.offspring_count())
                .map(|individual| individual.clone())
                .collect::<Population<TGene>>();

            for alterer in alterers {
                alterer.alter(&mut offspring);
            }

            population = Population::from_vec(survivors.individuals.into_iter()
                .chain(offspring.individuals.into_iter())
                .collect());
        }
    }
}