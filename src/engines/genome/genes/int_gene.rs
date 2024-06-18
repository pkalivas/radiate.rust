use rand::Rng;

use super::gene::{Allele, BoundGene, Gene, NumericGene};

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

impl Gene<IntGene> for IntGene {
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

    fn is_valid(&self) -> bool {
        if self.allele < self.min || self.allele > self.max {
            return false;
        }

        true
    }

    fn from_gene(&self, gene: &IntGene) -> IntGene {
        IntGene {
            allele: gene.allele,
            min: gene.min,
            max: gene.max,
            upper_bound: gene.upper_bound,
            lower_bound: gene.lower_bound,
        }
    }
}

impl Allele<i32> for IntGene {
    fn allele(&self) -> &i32 {
        &self.allele
    }
}

impl NumericGene<IntGene, i32> for IntGene {
    fn add(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            allele: self.allele + *other.allele(),
            ..*self
        }
    }

    fn sub(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            allele: self.allele - *other.allele(),
            ..*self
        }
    }

    fn mul(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            allele: self.allele * *other.allele(),
            ..*self
        }
    }

    fn div(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            allele: self.allele / *other.allele(),
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
