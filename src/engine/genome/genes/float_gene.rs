use rand::random;
use crate::engine::genome::gene::Gene;

#[derive(Debug)]
pub struct FloatGene {
    pub allele: f32
}

impl FloatGene {
    pub fn new() -> Self {
        FloatGene { allele: random::<f32>() }
    }
}

impl Gene<FloatGene, f32> for FloatGene {
    fn allele() -> f32 {
        Self.allele
    }

    fn new_instance() -> FloatGene {
        FloatGene::new()
    }

    fn from_value(value: f32) -> FloatGene {
        FloatGene { allele: value }
    }
}