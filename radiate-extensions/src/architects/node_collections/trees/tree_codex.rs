// use radiate_rust::engines::genome::genes::gene::Gene;
// use radiate_rust::engines::genome::genotype::Genotype;
// use radiate_rust::engines::genome::chromosome::Chromosome;
// use radiate_rust::engines::codexes::Codex;

use crate::architects::*;
// use crate::operations::op::Ops;


pub struct TreeCodex<'a, T>
where 
    T: Clone + PartialEq + Default
{
    pub input_size: usize,
    pub output_size: usize,
    pub factory: &'a NodeFactory<T>,
    pub nodes: Vec<Node<T>>,
}

impl<'a, T> TreeCodex<'a, T>
where
    T: Clone + PartialEq + Default
{
    // pub fn from_factory(factory: &'a NodeFactory<T>) -> Self {
    //     Self { 

    //     }
    // }

    // pub fn from shape(input_size: usize, output_size: usize, factory: &'a NodeFactory<T>) -> Self {
    //     Self {

    //     }
    // }

    pub fn from_nodes(nodes: Vec<Node<T>>, factory: &'a NodeFactory<T>) -> Self {
        Self {
            input_size: nodes
                .iter()
                .filter(|node| node.node_type == NodeType::Input)
                .count(),
            output_size: nodes
                .iter()
                .filter(|node| node.node_type == NodeType::Output)
                .count(),
            factory,
            nodes
        }
    }
}