use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::population::Population;
use crate::engines::genome::phenotype::Phenotype;

pub trait Codex<G, A, T>
where 
    G: Gene<G, A>
{
    fn encode(&self) -> Genotype<G, A>;

    fn decode(&self, genotype: &Genotype<G, A>) -> T;

    fn spawn(&self, num: i32) -> Vec<T> {
        (0..num)
            .into_iter()
            .map(|_| self.decode(&self.encode()))
            .collect::<Vec<T>>()
    }

    fn spawn_genotypes(&self, num: i32) -> Vec<Genotype<G, A>> {
        (0..num)
            .into_iter()
            .map(|_| self.encode())
            .collect::<Vec<Genotype<G, A>>>()
    }

    fn spawn_population(&self, num: i32) -> Population<G, A> {
        (0..num)
            .into_iter()
            .map(|_| Phenotype::from_genotype(self.encode(), 0))
            .collect::<Population<G, A>>()
    }
}

