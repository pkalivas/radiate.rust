use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct Mutator;

impl<TGene> Mutate<TGene> for Mutator where TGene: Gene<TGene> {}
