use crate::engines::genome::genes::gene::Gene;

use super::mutate::Mutate;

pub struct Mutator;

impl<G, A> Mutate<G, A> for Mutator where G: Gene<G, A> {}
