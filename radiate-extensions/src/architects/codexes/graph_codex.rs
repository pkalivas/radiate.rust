use std::sync::Arc;

use radiate_rust::engines::genome::genotype::Genotype;
use radiate_rust::engines::genome::chromosome::Chromosome;
use radiate_rust::engines::codexes::Codex;

use crate::architects::factories::node_factory::NodeFactory;
use crate::architects::node_collections::graph::Graph;
use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::nodes::node::Node;
use crate::architects::architect::Architect;


pub struct GraphCodex<'a, T> 
where
    T: Clone + PartialEq + Default
{
    pub input_size: usize,
    pub output_size: usize,
    pub factory: &'a NodeFactory<T>,
    pub nodes: Vec<Node<'a, T>>,
}

impl<'a, T> GraphCodex<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(input_size: usize, output_size: usize, factory: &'a NodeFactory<T>) -> GraphCodex<T> {
        let graph = Architect::<Graph<T>, T>::new(factory)
            .acyclic(input_size, output_size);

        GraphCodex { 
            input_size,
            output_size,
            factory, 
            nodes: graph
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<Node<T>>>()
        }
    }
}

impl<T> Codex<Node<T>, T, Graph<T>> for GraphCodex<T>
where
    T: Clone + PartialEq + Default
{
    fn encode(&self) -> Genotype<Node<T>, T> {
        Genotype {
            chromosomes: vec![Chromosome::from_genes(self.nodes
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<Node<T>>>())]
        }
    }

    fn decode(&self, genotype: &Genotype<Node<T>, T>) -> Graph<T> {
        Graph::from_nodes(genotype
            .iter()
            .next()
            .unwrap()
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>())
    }
}