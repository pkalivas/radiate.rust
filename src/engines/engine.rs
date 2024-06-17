use super::genome::population::Population;
use super::genome::genes::gene::Gene;


pub trait Engine<TGene, T> 
    where TGene : Gene<TGene>
{
    fn run(&self) -> EngineOutput<TGene, T>;
}

pub struct EngineOutput<TGene, T>
    where TGene : Gene<TGene>
{
    pub population: Population<TGene>,
    pub best: T,
    pub index: usize
}

impl<TGene, T> EngineOutput<TGene, T>
    where TGene : Gene<TGene>
{
    pub fn score(&self) -> f32 {
        match self.population.get(0).score() {
            Some(score) => score.to_float(),
            None => 0.0
        }
    }
}