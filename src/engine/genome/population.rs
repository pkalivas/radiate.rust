use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::phenotype::Phenotype;

pub struct Population<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub individuals: Vec<Phenotype<TGene, T>>,
    pub is_sorted: bool,
    phantom: std::marker::PhantomData<T>
}

impl<TGene, T> Population<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub fn new() -> Self {
        Population {
            individuals: Vec::new(),
            is_sorted: false,
            phantom: std::marker::PhantomData
        }
    }

    pub fn sort(&mut self) {
        self.individuals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.is_sorted = true;
    }

    pub fn add(&mut self, individual: Phenotype<TGene, T>) {
        self.individuals.push(individual);
        self.is_sorted = false;
    }

    pub fn get(&self, index: usize) -> Option<&Phenotype<TGene, T>> {
        self.individuals.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Phenotype<TGene, T>> {
        self.is_sorted = false;
        self.individuals.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn from_vec(individuals: Vec<Phenotype<TGene, T>>) -> Self {
        Population {
            individuals,
            is_sorted: false,
            phantom: std::marker::PhantomData
        }
    }

    pub fn from_func<F>(size: usize, f: F) -> Self
        where F: Fn() -> Phenotype<TGene, T>
    {
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            individuals.push(f());
        }

        Population {
            individuals,
            is_sorted: false,
            phantom: std::marker::PhantomData
        }
    }

}

impl<TGene, T> Clone for Population<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn clone(&self) -> Self {
        Population {
            individuals: self.individuals.clone(),
            is_sorted: self.is_sorted,
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene, T> std::fmt::Debug for Population<TGene, T>
    where TGene: Gene<TGene, T> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for individual in &self.individuals {
            write!(f, "{:?},\n ", individual)?;
        }
        write!(f, "]")
    }
}