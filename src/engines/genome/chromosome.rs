use super::genes::gene::Gene;

pub struct Chromosome<TGene>
where
    TGene: Gene<TGene>,
{
    pub genes: Vec<TGene>,
}

impl<TGene> Chromosome<TGene>
where
    TGene: Gene<TGene>,
{
    pub fn get_gene(&self, index: usize) -> &TGene {
        &self.genes[index]
    }

    pub fn set_gene(&mut self, index: usize, gene: TGene) {
        self.genes[index] = gene;
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> std::slice::Iter<TGene> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<TGene> {
        self.genes.iter_mut()
    }
}

impl<TGene> Clone for Chromosome<TGene>
where
    TGene: Gene<TGene>,
{
    fn clone(&self) -> Self {
        Chromosome {
            genes: self.genes.clone(),
        }
    }
}

impl<TGene> PartialEq for Chromosome<TGene>
where
    TGene: Gene<TGene>,
{
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.genes.iter().zip(other.genes.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl<TGene> std::fmt::Debug for Chromosome<TGene>
where
    TGene: Gene<TGene> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for gene in &self.genes {
            write!(f, "{:?}, ", gene)?;
        }
        write!(f, "]")
    }
}

