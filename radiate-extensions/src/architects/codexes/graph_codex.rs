
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
use crate::architects::schema::node_types::NodeType;
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
    pub fn from_factory(factory: NodeFactory<T>) -> GraphCodex<T> {
        GraphCodex::from_shape(1, 1, factory)
    }

    pub fn from_shape(input_size: usize, output_size: usize, factory: NodeFactory<T>) -> GraphCodex<T> {
        let nodes = Architect::<Graph<T>, T>::new(&factory)
            .acyclic(input_size, output_size)
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>();

        GraphCodex::from_nodes(nodes, factory)
    }

    pub fn from_nodes(nodes: Vec<Node<T>>, factory: NodeFactory<T>) -> GraphCodex<T> {
        GraphCodex {
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
        self.input_size = graph
            .iter()
            .filter(|node| node.node_type == NodeType::Input)
            .count();
        self.output_size = graph
            .iter()
            .filter(|node| node.node_type == NodeType::Output)
            .count();
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