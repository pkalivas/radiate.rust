use super::genes::gene::Gene;

pub struct Chromosome<G: Gene<G, A>, A>
{
    pub genes: Vec<G>,
    _allele: std::marker::PhantomData<A>,
}

impl<G: Gene<G, A>, A> Chromosome<G, A> {
    pub fn from_genes(genes: Vec<G>) -> Self {
        Chromosome {
            genes,
            _allele: std::marker::PhantomData,
        }
    }
    
    pub fn get_gene(&self, index: usize) -> &G {
        &self.genes[index]
    }

    pub fn set_gene(&mut self, index: usize, gene: G) {
        self.genes[index] = gene;
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_valid(&self) -> bool {
        for gene in &self.genes {
            if !gene.is_valid() {
                return false;
            }
        }

        true
    }

    pub fn iter(&self) -> std::slice::Iter<G> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<G> {
        self.genes.iter_mut()
    }
}

impl<G: Gene<G, A>, A> Clone for Chromosome<G, A> {
    fn clone(&self) -> Self {
        Chromosome {
            genes: self.genes.clone(),
            _allele: std::marker::PhantomData,
        }
    }
}

impl<G: Gene<G, A>, A> PartialEq for Chromosome<G, A> {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.genes.iter().zip(other.genes.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl<G: Gene<G, A> + std::fmt::Debug, A> std::fmt::Debug for Chromosome<G, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for gene in &self.genes {
            write!(f, "{:?}, ", gene)?;
        }
        write!(f, "]")
    }
}
