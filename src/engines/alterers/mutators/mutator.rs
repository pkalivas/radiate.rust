use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct Mutator;

impl<G: Gene<G, A>, A> Mutate<G, A> for Mutator {}
