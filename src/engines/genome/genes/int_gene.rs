use rand::Rng;

use super::gene::{BoundGene, Gene, NumericGene, Valid};

pub struct IntGene {
    allele: i32,
    min: i32,
    max: i32,
    upper_bound: i32,
    lower_bound: i32,
}

impl IntGene {
    pub fn new(min: i32, max: i32) -> Self {
        let (min, max) = if min > max { (max, min) } else { (min, max) };
        let mut rand = rand::thread_rng();
        IntGene {
            allele: rand.gen_range(min..max),
            min,
            max,
            upper_bound: std::i32::MAX,
            lower_bound: std::i32::MIN,
        }
    }
}

impl Gene<IntGene, i32> for IntGene {
    fn allele(&self) -> i32 {
        self.allele
    }

    fn new_instance(&self) -> IntGene {
        let mut rand = rand::thread_rng();
        IntGene {
            allele: rand.gen_range(self.min..self.max),
            min: self.min,
            max: self.max,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound,
        }
    }

    fn from_allele(&self, allele: &i32) -> IntGene {
        IntGene {
            allele: *allele,
            min: self.min,
            max: self.max,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound
        }
    }
}

impl Valid for IntGene {
    fn is_valid(&self) -> bool {
        self.allele >= self.min && self.allele <= self.max
    }
}

impl BoundGene<IntGene, i32> for IntGene {
    fn upper_bound(&self) -> &i32 {
        &self.upper_bound
    }

    fn lower_bound(&self) -> &i32 {
        &self.lower_bound
    }

    fn with_bounds(self, upper_bound: i32, lower_bound: i32) -> IntGene {
        IntGene {
            upper_bound,
            lower_bound,
            ..self
        }
    }
}

impl NumericGene<IntGene, i32> for IntGene {
    fn add(&self, other: &IntGene) -> IntGene {
        IntGene {
            allele: self.allele + other.allele,
            ..*self
        }
    }

    fn sub(&self, other: &IntGene) -> IntGene {
        IntGene {
            allele: self.allele - other.allele,
            ..*self
        }
    }

    fn mul(&self, other: &IntGene) -> IntGene {
        IntGene {
            allele: self.allele * other.allele,
            ..*self
        }
    }

    fn div(&self, other: &IntGene) -> IntGene {
        let other_allele = match other.allele() == 0_i32 {
            true => 1_i32,
            false => other.allele()
        };
        
        IntGene {
            allele: self.allele / other_allele,
            ..*self
        }
    }
}

impl Clone for IntGene {
    fn clone(&self) -> Self {
        IntGene {
            allele: self.allele,
            min: self.min,
            max: self.max,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound,
        }
    }
}

impl PartialEq for IntGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele
    }
}

impl std::fmt::Debug for IntGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.allele)
    }
}
