use super::gene::{Gene, Valid};

pub struct BitGene {
    allele: bool,
}

impl BitGene {
    pub fn new() -> Self {
        BitGene {
            allele: rand::random(),
        }
    }
}

impl Gene<BitGene, bool> for BitGene {
    fn allele(&self) -> bool {
        self.allele
    }

    fn new_instance(&self) -> BitGene {
        BitGene::new()
    }

    fn from_gene(&self, gene: &BitGene) -> BitGene {
        BitGene {
            allele: gene.allele,
        }
    }
}

impl Valid for BitGene {
    fn is_valid(&self) -> bool {
        true
    }
}

impl Clone for BitGene {
    fn clone(&self) -> Self {
        BitGene {
            allele: self.allele,
        }
    }
}

impl PartialEq for BitGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele
    }
}

impl std::fmt::Debug for BitGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.allele { 1 } else { 0 })
    }
}
