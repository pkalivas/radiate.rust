use crate::engines::genome::genes::gene::Gene;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::engine::Engine;
use crate::engines::genome::population::Population;
use crate::engines::codex::Codex;
use crate::engines::score::Score;
use crate::engines::alterers::alter::Alter;


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

    pub fn codex(&self) -> &Codex<TGene, T> {
        self.params.codex.as_ref().unwrap()
    }

    pub fn fitness_fn(&self) -> &dyn Fn(&T) -> Score {
        self.params.fitness_fn.as_ref().unwrap()
    }

    pub fn alters(&self) -> &Vec<Box<dyn Alter<TGene>>> {
        self.params.alterers.as_ref().unwrap()
    }

    pub fn population(&self) -> &Population<TGene> {
        self.params.population.as_ref().unwrap()
    }
}

impl<TGene, T> Engine<TGene> for GeneticEngine<TGene, T>
    where 
        TGene : Gene<TGene>,
        T : std::fmt::Debug
{
    fn run<TLimit>(&self, limit: TLimit)
        where TLimit: Fn(&Population<TGene>) -> bool
    {
        let mut population = self.population().clone();
        
        loop {

            for idx in 0..population.len() {
                let individual = population.get_mut(idx);
                if !individual.score.is_some() {
                    let decoded = self.codex().decode(individual.genotype());
                    let score = self.fitness_fn()(&decoded);

                    individual.score = Some(score);
                }
            }

            population.sort();

            if limit(&population) {
                break;
            }

            // let best_phenotype = population.get(0);
            // let decoded = self.codex().decode(&best_phenotype.genotype);

            // println!("{:?}: {:?}", i, decoded);

            let survivors = population.iter()
                .take(self.surivor_count())
                .map(|individual| individual.clone())
                .collect::<Population<TGene>>();

            let mut offspring = population.iter()
                .take(self.offspring_count())
                .map(|individual| individual.clone())
                .collect::<Population<TGene>>();

            for alterer in self.alters() {
                alterer.alter(&mut offspring);
            }

            population = Population::from_vec(survivors.into_iter()
                .chain(offspring.into_iter())
                .collect());
        }
    }
}