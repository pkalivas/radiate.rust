use super::{chromosome::Chromosome, genes::gene::Gene};

pub struct Genotype<G: Gene<G, A>, A> {
    pub chromosomes: Vec<Chromosome<G, A>>,
}

impl<G: Gene<G, A>, A> Genotype<G, A> {
    pub fn get_chromosome_mut(&mut self, index: usize) -> &mut Chromosome<G, A> {
        &mut self.chromosomes[index]
    }

    pub fn len(&self) -> usize {
        self.chromosomes.len()
    }

    pub fn is_valid(&self) -> bool {
        for chromosome in &self.chromosomes {
            if !chromosome.is_valid() {
                return false;
            }
        }

        true
    }

    pub fn iter(&self) -> std::slice::Iter<Chromosome<G, A>> {
        self.chromosomes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Chromosome<G, A>> {
        self.chromosomes.iter_mut()
    }
}

impl<G: Gene<G, A>, A> Clone for Genotype<G, A> {
    fn clone(&self) -> Self {
        Genotype {
            chromosomes: self.chromosomes.clone(),
        }
    }
}

impl<G: Gene<G, A>, A> PartialEq for Genotype<G, A> {
    fn eq(&self, other: &Self) -> bool {
        self.chromosomes == other.chromosomes
    }
}

impl<G: Gene<G, A> + std::fmt::Debug, A> std::fmt::Debug for Genotype<G, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for chromosome in &self.chromosomes {
            write!(f, "{:?},\n ", chromosome)?;
        }
        write!(f, "]")
    }
}