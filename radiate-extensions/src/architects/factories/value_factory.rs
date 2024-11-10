use std::collections::HashMap;
use rand::seq::SliceRandom;

use crate::architects::schema::node_types::NodeType;
use crate::architects::nodes::node::Node;

pub struct ValueFactory<T> {
    pub node_values: HashMap<NodeType, Vec<T>>
}

impl<T> ValueFactory<T> {
    pub fn new() -> ValueFactory<T> {
        ValueFactory {
            node_values: HashMap::new()
        }
    }

    pub fn inputs(mut self, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(NodeType::Input, values);
        self
    }

    pub fn outputs(mut self, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(NodeType::Output, values);
        self
    }
    
    pub fn gates(mut self, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(NodeType::Gate, values);
        self
    }
    
    pub fn aggregates(mut self, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(NodeType::Aggregate, values);
        self
    }
    
    pub fn weights(mut self, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(NodeType::Weight, values);
        self
    }

    pub fn set_values(mut self, node_type: NodeType, values: Vec<T>) -> ValueFactory<T> {
        self.add_node_values(node_type, values);
        self
    }

    pub fn add_node_values(&mut self, node_type: NodeType, values: Vec<T>) {
        self.node_values.insert(node_type, values);
    }

    pub fn new_node(&self, index: usize, node_type: NodeType) -> Node<T> 
    where
        T: Clone + PartialEq + Default
    {
        let mut rng = rand::thread_rng();
        if let Some(values) = self.node_values.get(&node_type) {
            let value = values.choose(&mut rng).unwrap();
            Node::new(index, node_type, value.clone())
        } else {
            Node::new(index, node_type, T::default())
        }
    }
}