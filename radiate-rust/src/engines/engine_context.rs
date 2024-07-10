use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::population::Population;
use crate::engines::schema::timer::Timer;

pub struct EngineContext<G: Gene<G, A>, A, T> {
    pub population: Population<G, A>,
    pub best: T,
    pub index: i32,
    pub timer: Timer,
}

impl<G: Gene<G, A>, A, T> EngineContext<G, A, T> {
    pub fn score(&self) -> f32 {
        match self.population.get(0).score() {
            Some(score) => score.as_float(),
            None => 0.0,
        }
    }
}

impl<G: Gene<G, A>, A, T: Clone> Clone for EngineContext<G, A, T> {
    fn clone(&self) -> Self {
        EngineContext {
            population: self.population.clone(),
            best: self.best.clone(),
            index: self.index,
            timer: self.timer.clone(),
        }
    }
}

impl<G: Gene<G, A>, A, T: std::fmt::Debug> std::fmt::Debug for EngineContext<G, A, T> {
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
