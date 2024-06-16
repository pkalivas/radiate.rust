use std::fmt::Debug;
use rand::random;

use crate::engines::genome::genes::gene::{Gene, Allele, NumericGene, BoundGene};

pub struct FloatGene {
    pub allele: f32,
    pub min: f32,
    pub max: f32
}

impl FloatGene {
    pub fn new(min: f32, max: f32) -> Self {
        FloatGene { 
            allele: random::<f32>() * (max - min) + min,
            min,
            max
         }
    }
}

impl Gene<FloatGene> for FloatGene {
    fn new_instance(&self) -> FloatGene {
        FloatGene::new(self.min, self.max)
    }

    fn is_valid(&self) -> bool {
        if self.allele < self.min || self.allele > self.max {
            return false;
        }

        true
    }

    fn from_gene(gene: &FloatGene) -> FloatGene {
        FloatGene { 
            allele: gene.allele,
            min: gene.min,
            max: gene.max
        }
    }
}

impl Allele<f32> for FloatGene {
    fn allele(&self) -> &f32 {
        &self.allele
    }
}

impl NumericGene<FloatGene, f32> for FloatGene {
    fn add(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele + *other.allele(), ..*self }
    }

    fn sub(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele - *other.allele(), ..*self }
    }

    fn mul(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele * *other.allele(), ..*self}
    }

    fn div(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene { allele: self.allele / *other.allele(), ..*self}
    }
}

impl BoundGene<FloatGene, f32> for FloatGene {
    fn min() -> f32 {
        f32::MIN
    }

    fn max() -> f32 {
        f32::MAX
    }
}

impl Clone for FloatGene {
    fn clone(&self) -> Self {
        FloatGene { 
            allele: self.allele,
            min: self.min,
            max: self.max
        }
    }
}

impl PartialEq for FloatGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele && self.min == other.min && self.max == other.max
    }
}

impl Debug for FloatGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.allele)
    }
}