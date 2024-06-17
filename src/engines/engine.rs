use super::genome::genes::gene::Gene;
use super::genome::population::Population;

pub trait Engine<TGene, T>
where
    TGene: Gene<TGene>,
{
    fn fit<F>(&self, limit: F) -> EngineOutput<TGene, T>
    where
        F: Fn(&EngineOutput<TGene, T>) -> bool;
}

pub struct EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub population: Population<TGene>,
    pub best: T,
    pub index: usize,
}

impl<TGene, T> EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub fn score(&self) -> f32 {
        match self.population.get(0).score() {
            Some(score) => score.to_float(),
            None => 0.0,
        }
    }
}


impl<TGene, T> std::fmt::Debug for EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EngineOutput {{\n")?;
        write!(f, "  best: {:?},\n", self.best)?;
        write!(f, "  score: {:?},\n", self.score())?;
        write!(f, "  index: {:?},\n", self.index)?;
        write!(f, "  size: {:?},\n", self.population.len())?;
        write!(f, "}}")
    }
    
}