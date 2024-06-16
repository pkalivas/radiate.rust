use crate::engine::genome::genes::gene::Gene;

pub struct Chromosome<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub genes: Vec<TGene>,
    phantom: std::marker::PhantomData<T>
}

impl<TGene, T> Chromosome<TGene, T>
    where TGene: Gene<TGene, T>
{
    pub fn from_slice(genes: &[TGene]) -> Self {
        Chromosome {
            genes: genes.to_vec(),
            phantom: std::marker::PhantomData
        }
    }

    pub fn from_vec(genes: Vec<TGene>) -> Self {
        Chromosome {
            genes,
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene, T> Clone for Chromosome<TGene, T>
    where TGene: Gene<TGene, T>
{
    fn clone(&self) -> Self {
        Chromosome {
            genes: self.genes.clone(),
            phantom: std::marker::PhantomData
        }
    }
}

impl<TGene, T> PartialEq for Chromosome<TGene, T>
    where TGene: Gene<TGene, T>
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

impl<TGene, T> std::fmt::Debug for Chromosome<TGene, T>
    where TGene: Gene<TGene, T> + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for gene in &self.genes {
            write!(f, "{:?}, ", gene)?;
        }
        write!(f, "]")
    }
}