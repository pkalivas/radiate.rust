
use radiate_rust::engines::genome::genes::gene::Gene;
use radiate_rust::engines::genome::genotype::Genotype;
use radiate_rust::engines::genome::chromosome::Chromosome;
use radiate_rust::engines::codexes::Codex;

use crate::architects::factories::node_factory::NodeFactory;
use crate::architects::node_collection_builder::NodeCollectionBuilder;
use crate::architects::node_collections::graph::Graph;
use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::nodes::node::Node;
use crate::architects::architect::Architect;
use crate::operations::op::Ops;


pub struct GraphCodex<T> 
where
    T: Clone + PartialEq + Default
{
    pub input_size: usize,
    pub output_size: usize,
    pub factory: NodeFactory<T>,
    pub nodes: Vec<Node<T>>,
}

impl<T> GraphCodex<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(input_size: usize, output_size: usize, factory: NodeFactory<T>) -> GraphCodex<T> {
        let graph = Architect::<Graph<T>, T>::new(&factory)
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

    pub fn set_nodes<F>(mut self, node_fn: F) -> Self
    where
        F: Fn(&Architect<Graph<T>, T>, NodeCollectionBuilder<Graph<T>, T>) -> Graph<T>
    {
        let graph = Architect::<Graph<T>, T>::new(&self.factory)
            .build(|arc, builder| node_fn(arc, builder));

        self.nodes = graph
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>();
        self
    }
}

impl<T> Codex<Node<T>, Ops<T>, Graph<T>> for GraphCodex<T>
where
    T: Clone + PartialEq + Default
{
    fn encode(&self) -> Genotype<Node<T>, Ops<T>> {
        Genotype {
            chromosomes: vec![Chromosome::from_genes(self.nodes
                .iter()
                .map(|node| node.new_instance())
                .collect::<Vec<Node<T>>>())]
        }
    }

    fn decode(&self, genotype: &Genotype<Node<T>, Ops<T>>) -> Graph<T> {
        Graph::from_nodes(genotype
            .iter()
            .next()
            .unwrap()
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>())
    }
}