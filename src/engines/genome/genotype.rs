use super::{chromosome::Chromosome, genes::gene::Gene};

pub struct Genotype<TGene: Gene<TGene>> {
    pub chromosomes: Vec<Chromosome<TGene>>,
}

impl<TGene: Gene<TGene>> Genotype<TGene> {
    pub fn get_chromosome_mut(&mut self, index: usize) -> &mut Chromosome<TGene> {
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

    pub fn iter(&self) -> std::slice::Iter<Chromosome<TGene>> {
        self.chromosomes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Chromosome<TGene>> {
        self.chromosomes.iter_mut()
    }
}

impl<TGene: Gene<TGene>> Clone for Genotype<TGene> {
    fn clone(&self) -> Self {
        Genotype {
            chromosomes: self.chromosomes.clone(),
        }
    }
}

impl<TGene: Gene<TGene>> PartialEq for Genotype<TGene> {
    fn eq(&self, other: &Self) -> bool {
        self.chromosomes == other.chromosomes
    }
}

impl<TGene: Gene<TGene> + std::fmt::Debug> std::fmt::Debug for Genotype<TGene> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for chromosome in &self.chromosomes {
            write!(f, "{:?},\n ", chromosome)?;
        }
        write!(f, "]")
    }
}
