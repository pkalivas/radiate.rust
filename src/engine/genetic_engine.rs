use crate::engine::genome::genes::gene::Gene;

pub struct GeneticEngine { }

impl GeneticEngine {
    pub fn new() -> Self {
        GeneticEngine { }
    }
}

// pub struct GeneticEngine<TGene, T>
//     where TGene : Gene<TGene, T> { }
//
// impl<TGene, T> GeneticEngine<TGene, T>
//     where TGene : Gene<TGene, T> {
//
//     pub fn new() -> Self {
//         GeneticEngine {
//
//         }
//     }
//
// }