use crate::engines::genome::genes::gene::Gene;
use crate::engines::genome::phenotype::Phenotype;

pub struct Population<TGene>
    where TGene: Gene<TGene>
{
    pub individuals: Vec<Phenotype<TGene>>,
    pub is_sorted: bool
}

impl<TGene> Population<TGene>
    where TGene: Gene<TGene>
{
    pub fn new() -> Self {
        Population {
            individuals: Vec::new(),
            is_sorted: false
        }
    }

    pub fn sort(&mut self) {
        self.individuals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.is_sorted = true;
    }

    pub fn add(&mut self, individual: Phenotype<TGene>) {
        self.individuals.push(individual);
        self.is_sorted = false;
    }

    pub fn get(&self, index: usize) -> Option<&Phenotype<TGene>> {
        self.individuals.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Phenotype<TGene>> {
        self.is_sorted = false;
        self.individuals.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn from_vec(individuals: Vec<Phenotype<TGene>>) -> Self {
        Population {
            individuals,
            is_sorted: false
        }
    }

    pub fn from_func<F>(size: usize, f: F) -> Self
        where F: Fn() -> Phenotype<TGene>
    {
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            individuals.push(f());
        }

        Population {
            individuals,
            is_sorted: false,
        }
    }

}

impl<TGene> Clone for Population<TGene>
    where TGene: Gene<TGene>
{
    fn clone(&self) -> Self {
        Population {
            individuals: self.individuals.clone(),
            is_sorted: self.is_sorted
        }
    }
}

impl<TGene> std::fmt::Debug for Population<TGene>
    where TGene: Gene<TGene> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for individual in &self.individuals {
            write!(f, "{:?},\n ", individual)?;
        }
        write!(f, "]")
    }
}