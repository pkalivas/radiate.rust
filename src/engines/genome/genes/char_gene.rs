use crate::engines::genome::genes::gene::{Allele, Gene};

const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"$%&/()=?`{[]}\\+~*#';.:,-_<>|@^' ";

pub struct CharGene {
    pub allele: char,
}

impl CharGene {
    pub fn new() -> Self {
        let index = rand::random::<usize>() % ALPHABET.len();
        CharGene {
            allele: ALPHABET.chars().nth(index).unwrap(),
        }
    }
}

impl Gene<CharGene> for CharGene {
    fn new_instance(&self) -> CharGene {
        CharGene::new()
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn from_gene(&self, gene: &CharGene) -> CharGene {
        CharGene {
            allele: gene.allele,
        }
    }
}

impl Allele<char> for CharGene {
    fn allele(&self) -> &char {
        &self.allele
    }
}

impl Clone for CharGene {
    fn clone(&self) -> Self {
        CharGene {
            allele: self.allele,
        }
    }
}

impl PartialEq for CharGene {
    fn eq(&self, other: &Self) -> bool {
        self.allele == other.allele
    }
}

impl std::fmt::Debug for CharGene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.allele)
    }
}

