use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::architects::schema::node_types::NodeType;
use crate::architects::nodes::node::Node;
use crate::architects::factories::node_factory::NodeFactory;
use crate::operations::op::{self, Ops};


pub struct OpFactory<T> 
where
    T: Clone + PartialEq + Default
{
    pub valeus: HashMap<NodeType, Vec<Ops<T>>>
}

impl<T> OpFactory<T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new() -> OpFactory<T> {
        OpFactory {
            valeus: HashMap::new()
        }
    }

    pub fn regression(input_size: usize) -> OpFactory<f32> {
        OpFactory::new()
            .inputs((0..input_size)
                .map(|idx| op::var(idx))
                .collect::<Vec<Ops<f32>>>())
            .gates(vec![
                op::add(),
                op::sub(),
                op::mul(),
                op::div(),
                op::pow(),
            ])
            .aggregates(vec![
                op::sigmoid(),
                op::tanh(),
                op::relu(),
                op::linear(),
                op::sum(),
                op::prod(),
            ])
            .weights(vec![op::weight()])
            .outputs(vec![op::linear()])

    }

    pub fn inputs(mut self, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(NodeType::Input, values);
        self
    }

    pub fn outputs(mut self, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(NodeType::Output, values);
        self
    }
    
    pub fn gates(mut self, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(NodeType::Gate, values);
        self
    }
    
    pub fn aggregates(mut self, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(NodeType::Aggregate, values);
        self
    }
    
    pub fn weights(mut self, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(NodeType::Weight, values);
        self
    }

    pub fn set_values(mut self, node_type: NodeType, values: Vec<Ops<T>>) -> OpFactory<T> {
        self.add_node_values(node_type, values);
        self
    }

    pub fn add_node_values(&mut self, node_type: NodeType, values: Vec<Ops<T>>) {
        self.valeus.insert(node_type, values);
    }
}

impl<T> NodeFactory<Ops<T>> for OpFactory<T> 
where
    T: Clone + PartialEq + Default
{
    fn new_node(&self, index: usize, node_type: NodeType) -> Node<Ops<T>> 
    where
        T: Clone + PartialEq + Default
    {
        let mut rng = rand::thread_rng();
        if let Some(values) = self.valeus.get(&node_type) {
            match node_type {
                NodeType::Input => {
                    let value = values[index % values.len()].clone();
                    return Node::new(index, node_type, value.new_instance());
                },
                _ => {
                    let value = values.choose(&mut rng).unwrap();
                    return Node::new(index, node_type, value.new_instance());
                }
            }
        }
        
        Node::new(index, node_type, Ops::default())
    }
}