use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::schema::timer::Timer;

pub struct EngineHandle<TGene: Gene<TGene>, T> {
    pub population: Population<TGene>,
    pub best: T,
    pub index: i32,
    pub timer: Timer,
}

impl<TGene: Gene<TGene>, T> EngineHandle<TGene, T> {
    pub fn score(&self) -> f32 {
        match self.population.get(0).score() {
            Some(score) => score.as_float(),
            None => 0.0,
        }
    }
}

impl<TGene: Gene<TGene>, T: Clone> Clone for EngineHandle<TGene, T> {
    fn clone(&self) -> Self {
        EngineHandle {
            population: self.population.clone(),
            best: self.best.clone(),
            index: self.index,
            timer: self.timer.clone(),
        }
    }
}

impl<TGene: Gene<TGene>, T: std::fmt::Debug> std::fmt::Debug for EngineHandle<TGene, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EngineOutput {{\n")?;
        write!(f, "  best: {:?},\n", self.best)?;
        write!(f, "  score: {:?},\n", self.score())?;
        write!(f, "  index: {:?},\n", self.index)?;
        write!(f, "  size: {:?},\n", self.population.len())?;
        write!(f, "  duration: {:?},\n", self.timer.elapsed())?;
        write!(f, "}}")
    }
}
