use std::fmt::Debug;
use rand::random;

use crate::engine::genome::genes::gene::Gene;

pub struct FloatGene {
    pub allele: f32
}

impl FloatGene {
    pub fn new() -> Self {
        FloatGene { allele: random::<f32>() }
    }
}

impl Gene<FloatGene, f32> for FloatGene {
    
    fn allele(&self) -> &f32 {
        &self.allele
    }

    fn new_instance() -> FloatGene {
        FloatGene::new()
    }

    fn from_value(value: f32) -> FloatGene {
        FloatGene { allele: value }
    }
}

impl Clone for FloatGene {
    fn clone(&self) -> Self {
        FloatGene { allele: self.allele }
    }
}

impl PartialEq for FloatGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele
    }
}

impl Debug for FloatGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.allele)
    }
}