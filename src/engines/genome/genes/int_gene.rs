use rand::Rng;

use crate::engines::genome::genes::gene::{Gene, Allele, NumericGene, BoundGene};


pub struct IntGene {
    value: i32,
    min: i32,
    max: i32,
}

impl IntGene {
    pub fn new(min: i32, max: i32) -> Self {
        let (min, max) = if min > max { (max, min) } else { (min, max) };
        let mut rand = rand::thread_rng();

        IntGene {
            value: rand.gen_range(min..max),
            min,
            max,
        }
    }
}

impl Gene<IntGene> for IntGene {
    fn new_instance(&self) -> IntGene {
        IntGene::new(self.min, self.max)
    }

    fn is_valid(&self) -> bool {
        if self.value < self.min || self.value > self.max {
            return false;
        }

        true
    }

    fn from_gene(&self, gene: &IntGene) -> IntGene {
        IntGene {
            value: gene.value,
            min: gene.min,
            max: gene.max,
        }
    }
}

impl Allele<i32> for IntGene {
    fn allele(&self) -> &i32 {
        &self.value
    }
}

impl NumericGene<IntGene, i32> for IntGene {
    fn add(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            value: self.value + *other.allele(),
            ..*self
        }
    }

    fn sub(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            value: self.value - *other.allele(),
            ..*self
        }
    }

    fn mul(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            value: self.value * *other.allele(),
            ..*self
        }
    }

    fn div(&self, other: &impl NumericGene<IntGene, i32>) -> IntGene {
        IntGene {
            value: self.value / *other.allele(),
            ..*self
        }
    }
}

impl Clone for IntGene {
    fn clone(&self) -> Self {
        IntGene {
            value: self.value,
            min: self.min,
            max: self.max,
        }
    }
}

impl BoundGene<IntGene, i32> for IntGene {
    fn min(&self) -> &i32 {
        &self.min
    }

    fn max(&self) -> &i32 {
        &self.max
    }
}

impl PartialEq for IntGene {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::fmt::Debug for IntGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}