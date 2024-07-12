use crate::engines::codex::Codex;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::score::Score;


pub trait Problem<G: Gene<G, A>, A, T> {
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score;
    fn codex(&self) -> &Codex<G, A, T>;
}

pub struct DefaultProblem<'a, G: Gene<G, A>, A, T> {
    pub fitness_fn: &'a dyn Fn(&T) -> Score,
    pub codex: Codex<G, A, T>
}

impl<'a, G: Gene<G, A>, A, T> Problem<G, A, T> for DefaultProblem<'a, G, A, T> {
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score {
        let decoded = self.codex.decode(genotype);
        (self.fitness_fn)(&decoded)
    }

    fn codex(&self) -> &Codex<G, A, T> {
        &self.codex
    }
}