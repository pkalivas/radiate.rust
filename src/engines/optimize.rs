use super::genome::{genes::gene::Gene, population::Population};

pub enum Optimize {
    Minimize,
    Maximize,
}

impl Optimize {
    pub fn sort<TGene>(&self, population: &mut Population<TGene>)
    where
        TGene: Gene<TGene>
    {
        match self {
            Optimize::Minimize => population.sort_by(|a, b| a.partial_cmp(&b).unwrap()),
            Optimize::Maximize => population.sort_by(|a, b| b.partial_cmp(&a).unwrap()),
        }
    }

    pub fn sort_index<TGene>(&self, population: &mut Population<TGene>) -> Vec<usize>
    where
        TGene: Gene<TGene>
    {
        match self {
            Optimize::Minimize => {
                let mut indices: Vec<usize> = (0..population.len()).collect();
                indices.sort_by(|&a, &b| population.get(a).partial_cmp(&population.get(b)).unwrap());
                indices
            },
            Optimize::Maximize => {
                let mut indices: Vec<usize> = (0..population.len()).collect();
                indices.sort_by(|&a, &b| population.get(b).partial_cmp(&population.get(a)).unwrap());
                indices
            },
        }
    }


}