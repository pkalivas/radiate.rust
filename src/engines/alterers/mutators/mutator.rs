use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct Mutator;

impl<TGene: Gene<TGene>> Mutate<TGene> for Mutator {}
