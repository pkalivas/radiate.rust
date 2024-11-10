use super::genome::{genes::gene::Gene, population::Population};

pub enum Optimize {
    Minimize,
    Maximize,
}

impl Optimize {
    pub fn sort<G, A>(&self, population: &mut Population<G, A>)
    where
        G: Gene<G, A>
    {
        match self {
            Optimize::Minimize => population.sort_by(|a, b| a.partial_cmp(&b).unwrap()),
            Optimize::Maximize => population.sort_by(|a, b| b.partial_cmp(&a).unwrap()),
        }
    }
}
