use std::array::IntoIter;

use crate::engines::genome::genes::gene::Gene;
use crate::engines::genetic_engine_params::GeneticEngineParams;
use crate::engines::engine::Engine;
use crate::engines::genome::population::Population;
use crate::engines::codex::Codex;
use crate::engines::score::Score;
use crate::engines::alterers::alter::Alter;
use crate::engines::engine::EngineOutput;


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

    pub fn evaluate(&self, population: &mut Population<TGene>) {
        for idx in 0..population.len() {
            let individual = population.get_mut(idx);
            if !individual.score.is_some() {
                let decoded = self.codex().decode(individual.genotype());
                let score = self.fitness_fn()(&decoded);

                individual.score = Some(score);
            }
        }

        population.sort();
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

impl<TGene, T> Engine<TGene, T> for GeneticEngine<TGene, T>
    where TGene : Gene<TGene>
{
    fn run(&self) -> EngineOutput<TGene, T> {
        let mut population = self.population().clone();

        for _ in 0..200 {
            self.evaluate(&mut population);

            let suvivors = population.iter()
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
    
            population = Population::from_vec(suvivors.into_iter()
                .chain(offspring.into_iter())
                .collect());
        }
        
        let best = self.codex().decode(&population.get(0).genotype()); 

        EngineOutput {
            population: population.clone(),
            best,
            index: 0
        }
    }
}




// impl<TGene, T> std::iter::Iterator for GeneticEngine<TGene, T>
//     where TGene : Gene<TGene>
// {
//     type Item = EngineOutput<TGene, T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let output = EngineOutput {
//             population: self.population().clone(),
//             best: self.codex().decode(&self.population().get(0).genotype()),
//             index: 0
//         };

//         Some(self.run(output))
//     }
// }

// let mut population = self.population().clone();
        
// loop {
//     for idx in 0..population.len() {
//         let individual = population.get_mut(idx);
//         if !individual.score.is_some() {
//             let decoded = self.codex().decode(individual.genotype());
//             let score = self.fitness_fn()(&decoded);

//             individual.score = Some(score);
//         }
//     }

//     population.sort();

//     if limit(&population) {
//         break;
//     }

//     let survivors = population.iter()
//         .take(self.surivor_count())
//         .map(|individual| individual.clone())
//         .collect::<Population<TGene>>();

//     let mut offspring = population.iter()
//         .take(self.offspring_count())
//         .map(|individual| individual.clone())
//         .collect::<Population<TGene>>();

//     for alterer in self.alters() {
//         alterer.alter(&mut offspring);
//     }

//     population = Population::from_vec(survivors.into_iter()
//         .chain(offspring.into_iter())
//         .collect());
// }