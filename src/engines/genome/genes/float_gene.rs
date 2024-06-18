use rand::random;

use super::gene::{Allele, BoundGene, Gene, NumericGene};

pub struct FloatGene {
    pub allele: f32,
    pub min: f32,
    pub max: f32,
    pub upper_bound: f32,
    pub lower_bound: f32,
}

impl FloatGene {
    pub fn new(min: f32, max: f32) -> Self {
        FloatGene {
            allele: random::<f32>() * (max - min) + min,
            min,
            max,
            upper_bound: f32::MAX,
            lower_bound: f32::MIN,
        }
    }
}

impl Gene<FloatGene> for FloatGene {
    fn new_instance(&self) -> FloatGene {
        FloatGene {
            allele: random::<f32>() * (self.max - self.min) + self.min,
            min: self.min,
            max: self.max,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound,
        }
    }

    fn is_valid(&self) -> bool {
        if self.allele > self.upper_bound || self.allele < self.lower_bound {
            return false;
        }

        true
    }

    fn from_gene(&self, gene: &FloatGene) -> FloatGene {
        FloatGene {
            allele: gene.allele,
            min: gene.min,
            max: gene.max,
            upper_bound: gene.upper_bound,
            lower_bound: gene.lower_bound,
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
        FloatGene {
            allele: self.allele + *other.allele(),
            ..*self
        }
    }

    fn sub(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene {
            allele: self.allele - *other.allele(),
            ..*self
        }
    }

    fn mul(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene {
            allele: self.allele * *other.allele(),
            ..*self
        }
    }

    fn div(&self, other: &impl NumericGene<FloatGene, f32>) -> FloatGene {
        FloatGene {
            allele: self.allele / *other.allele(),
            ..*self
        }
    }
}

impl BoundGene<FloatGene, f32> for FloatGene {
    fn upper_bound(&self) -> &f32 {
        &self.upper_bound
    }

    fn lower_bound(&self) -> &f32 {
        &self.lower_bound
    }

    fn with_bounds(self, upper_bound: f32, lower_bound: f32) -> FloatGene {
        FloatGene {
            upper_bound,
            lower_bound,
            ..self
        }
    }
}

impl Clone for FloatGene {
    fn clone(&self) -> Self {
        FloatGene {
            allele: self.allele,
            min: self.min,
            max: self.max,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound,
        }
    }
}

impl PartialEq for FloatGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele && self.min == other.min && self.max == other.max
    }
}

impl std::fmt::Debug for FloatGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.allele)
    }
}
