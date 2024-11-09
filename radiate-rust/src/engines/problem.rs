use std::sync::Arc;

use crate::engines::codex::Codex;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::score::Score;


pub trait Problem<G, A, T> 
    where G: Gene<G, A>
{
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score;
    fn codex(&self) -> &Codex<G, A, T>;
}

pub struct DefaultProblem<G, A, T> 
    where G: Gene<G, A>
{
    pub fitness_fn: Arc<dyn Fn(&T) -> Score>,
    pub codex: Arc<Codex<G, A, T>>
}

impl<G, A, T> Problem<G, A, T> for DefaultProblem<G, A, T>
    where G: Gene<G, A>
{
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score {
        let decoded = self.codex.decode(genotype);
        (self.fitness_fn)(&decoded)
    }

    fn codex(&self) -> &Codex<G, A, T> {
        &self.codex
    }
}