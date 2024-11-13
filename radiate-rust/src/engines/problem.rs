use std::sync::Arc;

use super::{codexes::Codex, Gene, Genotype, Phenotype, Score};


pub trait Problem<G, A, T>
where
    G: Gene<G, A> 
{
    fn eval(&self, phenotype: &mut Phenotype<G, A>);
    fn encode(&self) -> Genotype<G, A>;
    fn decode(&self, genotype: &Genotype<G, A>) -> T;
}


pub struct DefaultProblem<'a, G, A, T>
where
    G: Gene<G, A> 
{
    pub codex: Arc<&'a dyn Codex<G, A, T>>,
    pub fitness_fn: Arc<dyn Fn(&T) -> Score>,
}

impl<'a, G, A, T> DefaultProblem<'a, G, A, T>
where
    G: Gene<G, A>
{
    pub fn new(codex: Arc<&'a dyn Codex<G, A, T>>, fitness_fn: Arc<dyn Fn(&T) -> Score>) -> Self {
        DefaultProblem {
            codex,
            fitness_fn,
        }
    }
}

impl<'a, G, A, T> Problem< G, A, T> for DefaultProblem<'a, G, A, T>
where
    G: Gene<G, A>
{
    fn eval(&self, phenotype: &mut Phenotype<G, A>) {
        let genotype = phenotype.genotype();
        let decoded = self.decode(genotype);
        let score = (self.fitness_fn)(&decoded);
    
        phenotype.set_score(Some(score));
    }

    fn encode(&self) -> Genotype<G, A> {
        self.codex.encode()
    }

    fn decode(&self, genotype: &Genotype<G, A>) -> T {
        self.codex.decode(genotype)
    }
}