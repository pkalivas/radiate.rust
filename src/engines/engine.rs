use super::genome::genes::gene::Gene;
use super::genome::population::Population;
use super::schema::timer::Timer;

pub trait Engine<TGene, T>
where
    TGene: Gene<TGene>,
    T: Clone,
{
    fn start(&self) -> EngineOutput<TGene, T>;
    fn fit<F>(&self, limit: F) -> EngineOutput<TGene, T>
    where
        F: Fn(&EngineOutput<TGene, T>) -> bool;

    fn stop(&self, output: &mut EngineOutput<TGene, T>) -> EngineOutput<TGene, T> {
        output.timer.stop();
        output.clone()
    }
}

pub struct EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub population: Population<TGene>,
    pub best: T,
    pub index: usize,
    pub timer: Timer,
}

impl<TGene, T> EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
{
    pub fn score(&self) -> f32 {
        match self.population.get(0).score() {
            Some(score) => score.as_float(),
            None => 0.0,
        }
    }
}

impl<TGene, T> Clone for EngineOutput<TGene, T>
where
    TGene: Gene<TGene>,
    T: Clone,
{
    fn clone(&self) -> Self {
        EngineOutput {
            population: self.population.clone(),
            best: self.best.clone(),
            index: self.index,
            timer: self.timer.clone(),
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
        write!(f, "  duration: {:?},\n", self.timer.elapsed())?;
        write!(f, "}}")
    }
}
