use crate::engine::genome::genes::gene::Gene;

pub struct Chromosome<TGene>
    where TGene: Gene<TGene>
{
    pub genes: Vec<TGene>,
}

impl<TGene> Chromosome<TGene>
    where TGene: Gene<TGene>
{
    pub fn from_slice(genes: &[TGene]) -> Self {
        Chromosome { genes: genes.to_vec() }
    }

    pub fn from_vec(genes: Vec<TGene>) -> Self {
        Chromosome { genes }
    }
}

impl<TGene> Clone for Chromosome<TGene>
    where TGene: Gene<TGene>
{
    fn clone(&self) -> Self {
        Chromosome { genes: self.genes.clone() }
    }
}

impl<TGene> PartialEq for Chromosome<TGene>
    where TGene: Gene<TGene>
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
    where TGene: Gene<TGene> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for gene in &self.genes {
            write!(f, "{:?}, ", gene)?;
        }
        write!(f, "]")
    }
}