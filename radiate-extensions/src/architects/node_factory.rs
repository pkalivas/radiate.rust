use std::collections::HashMap;
use rand::seq::SliceRandom;

use crate::architects::schema::node_types::NodeType;
use crate::architects::nodes::node::Node;


pub struct NodeFactory<T> {
    pub node_values: HashMap<NodeType, Vec<T>>
}

impl<T> NodeFactory<T> {
    pub fn new() -> NodeFactory<T> {
        NodeFactory {
            node_values: HashMap::new()
        }
    }

    pub fn inputs(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Input, values);
        self
    }

    pub fn outputs(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Output, values);
        self
    }
    
    pub fn gates(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Gate, values);
        self
    }
    
    pub fn aggregates(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Aggregate, values);
        self
    }
    
    pub fn weights(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Weight, values);
        self
    }

    pub fn set_values(mut self, node_type: NodeType, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(node_type, values);
        self
    }

    pub fn add_node_values(&mut self, node_type: NodeType, values: Vec<T>) {
        self.node_values.insert(node_type, values);
    }

    pub fn new_node<N>(&self, index: usize, node_type: NodeType) -> N
    where
        N: Node<N, T> + Clone + Default,
        T: Clone + PartialEq + Default
    {
        let mut rng = rand::thread_rng();
        if let Some(values) = self.node_values.get(&node_type) {
            let value = values.choose(&mut rng).unwrap();
            N::new_node(index, node_type, value.clone())
        } else {
            N::new_node(index, node_type, T::default())
        }
    }
}