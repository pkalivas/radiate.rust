use super::{genes::gene::Gene, phenotype::Phenotype};

pub struct Population<TGene: Gene<TGene>> {
    pub individuals: Vec<Phenotype<TGene>>,
    pub is_sorted: bool,
}

impl<TGene: Gene<TGene>> Population<TGene> {
    pub fn get(&self, index: usize) -> &Phenotype<TGene> {
        self.individuals.get(index).expect("Index out of bounds")
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Phenotype<TGene> {
        self.is_sorted = false;
        self.individuals
            .get_mut(index)
            .expect("Index out of bounds")
    }

    pub fn set(&mut self, index: usize, individual: Phenotype<TGene>) {
        self.individuals[index] = individual;
        self.is_sorted = false;
    }

    pub fn iter(&self) -> std::slice::Iter<Phenotype<TGene>> {
        self.individuals.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Phenotype<TGene>> {
        self.is_sorted = false;
        self.individuals.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn sort_by<F>(&mut self, f: F)
    where
        F: FnMut(&Phenotype<TGene>, &Phenotype<TGene>) -> std::cmp::Ordering,
    {
        if self.is_sorted {
            return;
        }

        self.individuals.sort_by(f);
        self.is_sorted = true;
    }

    pub fn from_vec(individuals: Vec<Phenotype<TGene>>) -> Self {
        Population {
            individuals,
            is_sorted: false,
        }
    }

    pub fn from_func<F>(size: usize, f: F) -> Self
    where
        F: Fn() -> Phenotype<TGene>,
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

impl<TGene: Gene<TGene>> std::iter::IntoIterator for Population<TGene> {

    type Item = Phenotype<TGene>;
    type IntoIter = std::vec::IntoIter<Phenotype<TGene>>;

    fn into_iter(self) -> Self::IntoIter {
        self.individuals.into_iter()
    }
}

impl<TGene: Gene<TGene>> std::iter::FromIterator<Phenotype<TGene>> for Population<TGene> {
    fn from_iter<I: IntoIterator<Item = Phenotype<TGene>>>(iter: I) -> Self {
        let individuals = iter.into_iter().collect();
        Population {
            individuals,
            is_sorted: false,
        }
    }
}

impl<TGene: Gene<TGene>> Clone for Population<TGene> {
    fn clone(&self) -> Self {
        Population {
            individuals: self.individuals.clone(),
            is_sorted: self.is_sorted,
        }
    }
}

impl<TGene: Gene<TGene> + std::fmt::Debug> std::fmt::Debug for Population<TGene> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for individual in &self.individuals {
            write!(f, "{:?},\n ", individual)?;
        }
        write!(f, "]")
    }
}
