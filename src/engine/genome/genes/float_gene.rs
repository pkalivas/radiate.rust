use std::fmt::Debug;
use rand::random;

use crate::engine::genome::genes::gene::{Gene, Allele, NumericGene};

pub struct FloatGene {
    pub allele: f32
}

impl FloatGene {
    pub fn new() -> Self {
        FloatGene { allele: random::<f32>() }
    }
}

impl Gene<FloatGene> for FloatGene {
    fn new_instance() -> FloatGene {
        FloatGene::new()
    }

    fn from_gene(gene: FloatGene) -> FloatGene {
        FloatGene { allele: gene.allele }
    }
}

impl Allele<f32> for FloatGene {
    fn allele(&self) -> &f32 {
        &self.allele
    }
}

impl NumericGene<FloatGene, f32> for FloatGene {
    fn add(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele + *other.allele() }
    }

    fn sub(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele - *other.allele() }
    }

    fn mul(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele * *other.allele() }
    }

    fn div(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele / *other.allele() }
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