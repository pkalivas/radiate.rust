use std::ops::{Mul, Sub, Add};

use num_traits::Float;
use rand::{prelude::Distribution, distributions::Standard, random};
use radiate_rust::engines::{alterers::mutators::mutate::Mutate, genome::genes::gene::Gene};

use crate::architects::node_collections::node::Node;
use crate::operations::op::Ops;


pub struct NodeMutator<T>
where
    Standard: Distribution<T>,
{
    pub rate: f32,
    pub replace_rate: f32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NodeMutator<T> 
where
    Standard: Distribution<T>,
{
    pub fn new(rate: f32, replace_rate: f32) -> Self {
        Self { 
            rate,
            replace_rate,
             _phantom: std::marker::PhantomData 
        }
    }
}

impl<T> Mutate<Node<T>, Ops<T>> for NodeMutator<T>
where
    T: Clone + PartialEq + Default + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Float,
    Standard: Distribution<T>,
{
    fn mutate_rate(&self) -> f32 {
        self.rate
    }

    fn mutate_gene(&self, gene: &Node<T>) -> Node<T> {
        match gene.allele() {
            Ops::MutableConst(name, arity, value, supplier, operation) => {
                let random_value = random::<T>() * T::from(2).unwrap() - T::from(1).unwrap();

                if random::<f32>() < self.replace_rate {
                    gene.from_allele(&Ops::MutableConst(&name, *arity, random_value, supplier.clone(), operation.clone()))
                } else {
                    let new_value = random_value + value.clone();
                    gene.from_allele(&Ops::MutableConst(&name, *arity, new_value, supplier.clone(), operation.clone()))
                }
            },
            _ => gene.from_allele(&gene.allele())
        }
    }
}
