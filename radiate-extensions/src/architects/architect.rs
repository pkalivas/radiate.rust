
use crate::architects::node_collections::node::Node;
use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::node_collection_builder::NodeCollectionBuilder;
use crate::architects::schema::node_types::NodeType;
use crate::architects::node_collections::node_factory::NodeFactory;

use super::Graph;


pub struct Architect<'a, C, T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub node_factory: &'a NodeFactory<T>,
    _phantom: std::marker::PhantomData<C>,
}

impl<'a, C, T> Architect<'a, C, T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new(node_factory: &'a NodeFactory<T>) -> Self {
        Architect {
            node_factory,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn build<F>(&self, build_fn: F) -> C
    where
        F: FnOnce(&Architect<C, T>, NodeCollectionBuilder<C, T>) -> C
    {
        build_fn(self, NodeCollectionBuilder::new(&self.node_factory))
    }

    pub fn input(&self, siez: usize) -> C {
        self.new_collection(NodeType::Input, siez)
    }

    pub fn output(&self, siez: usize) -> C {
        self.new_collection(NodeType::Output, siez)
    }

    pub fn gate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Gate, siez)
    }

    pub fn aggregate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Aggregate, siez)
    }

    pub fn weight(&self, siez: usize) -> C {
        self.new_collection(NodeType::Weight, siez)
    }

    pub fn new_collection(&self, node_type: NodeType, size: usize) -> C {
        let nodes = self.new_nodes(node_type, size);
        C::from_nodes(nodes)
    }

    pub fn new_nodes(&self, node_type: NodeType, size: usize) -> Vec<Node<T>> {
        (0..size)
            .map(|i| self.node_factory.new_node(i, node_type))
            .collect::<Vec<Node<T>>>()
    }

    pub fn acyclic(&self, input_size: usize, output_size: usize) -> Graph<T> {
        let graph_architect = Architect::<Graph<T>, T>::new(self.node_factory);
        graph_architect.build(|arc, builder| builder
            .one_to_one(&arc.input(input_size), &arc.output(output_size))
            .build())
    }

    pub fn cyclic(&self, input_size: usize, output_size: usize) -> Graph<T> {
        let graph_architect = Architect::<Graph<T>, T>::new(self.node_factory);
        graph_architect.build(|arc, builder| {
            let input = arc.input(input_size);
            let aggregate = arc.aggregate(input_size);
            let link = arc.gate(input_size);
            let output = arc.output(output_size);

            builder
                .one_to_one(&input, &aggregate)
                .one_to_one_self(&aggregate, &link)
                .all_to_all(&aggregate, &output)
                .build()
        })
    }

    pub fn weighted_acyclic(&self, input_size: usize, output_size: usize) -> Graph<T> {
        let graph_architect = Architect::<Graph<T>, T>::new(self.node_factory);
        graph_architect.build(|arc, builder| {
            let input = arc.input(input_size);
            let output = arc.output(output_size);
            let weights = arc.weight(input_size * output_size);

            builder
                .one_to_many(&input, &weights)
                .many_to_one(&weights, &output)
                .build()
        })
    }

    pub fn weighted_cyclic(&self, input_size: usize, output_size: usize, memory_size: usize) -> Graph<T> {
        let graph_architect = Architect::<Graph<T>, T>::new(self.node_factory);
        graph_architect.build(|arc, builder| {
            let input = arc.input(input_size);
            let output = arc.output(output_size);
            let weights = arc.weight(input_size * memory_size);
            let aggregate = arc.aggregate(memory_size);
            let aggregate_weights = arc.weight(memory_size);

            builder
                .one_to_many(&input, &weights)
                .many_to_one(&weights, &aggregate)
                .one_to_one_self(&aggregate, &aggregate_weights)
                .all_to_all(&aggregate, &output)
                .build()
        })
    }
}


pub trait Arch<C, T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn get_factory(&self) -> &NodeFactory<T>;

    fn build<F, A>(&self, build_fn: F) -> C
    where
        F: FnOnce(&A, NodeCollectionBuilder<C, T>) -> C,
        A: Arch<C, T>;

    fn input(&self, siez: usize) -> C {
        self.new_collection(NodeType::Input, siez)
    }

    fn output(&self, siez: usize) -> C {
        self.new_collection(NodeType::Output, siez)
    }

    fn gate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Gate, siez)
    }

    fn aggregate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Aggregate, siez)
    }

    fn weight(&self, siez: usize) -> C {
        self.new_collection(NodeType::Weight, siez)
    }

    fn new_collection(&self, node_type: NodeType, size: usize) -> C {
        let nodes = self.new_nodes(node_type, size);
        C::from_nodes(nodes)
    }

    fn new_nodes(&self, node_type: NodeType, size: usize) -> Vec<Node<T>> {
        (0..size)
            .map(|i| self.get_factory().new_node(i, node_type))
            .collect::<Vec<Node<T>>>()
    }
}