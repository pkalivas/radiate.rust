use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::{architects::{nodes::node::Node, schema::node_types::NodeType}, operations::op::{self, Ops}};


// pub trait NodeFactory<T> {
//     fn new_node(&self, index: usize, node_type: NodeType) -> Node<T> 
//     where
//         T: Clone + PartialEq + Default;
// }


pub struct NodeFactory<T> 
where
    T: Clone + PartialEq + Default
{
    pub node_values: HashMap<NodeType, Vec<Ops<T>>>
}

impl<T> NodeFactory<T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new() -> NodeFactory<T> {
        NodeFactory {
            node_values: HashMap::new()
        }
    }

    pub fn inputs(mut self, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Input, values);
        self
    }

    pub fn input_values(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Input, values.iter().map(|v| op::value(v.clone())).collect());
        self
    }

    pub fn outputs(mut self, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Output, values);
        self
    }

    pub fn output_values(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Output, values.iter().map(|v| op::value(v.clone())).collect());
        self
    }
    
    pub fn gates(mut self, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Gate, values);
        self
    }

    pub fn gate_values(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Gate, values.iter().map(|v| op::value(v.clone())).collect());
        self
    }
    
    pub fn aggregates(mut self, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Aggregate, values);
        self
    }

    pub fn aggregate_values(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Aggregate, values.iter().map(|v| op::value(v.clone())).collect());
        self
    }
    
    pub fn weights(mut self, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Weight, values);
        self
    }

    pub fn weight_values(mut self, values: Vec<T>) -> NodeFactory<T> {
        self.add_node_values(NodeType::Weight, values.iter().map(|v| op::value(v.clone())).collect());
        self
    }

    pub fn set_values(mut self, node_type: NodeType, values: Vec<Ops<T>>) -> NodeFactory<T> {
        self.add_node_values(node_type, values);
        self
    }

    pub fn add_node_values(&mut self, node_type: NodeType, values: Vec<Ops<T>>) {
        self.node_values.insert(node_type, values);
    }

    pub fn new_node(&self, index: usize, node_type: NodeType) -> Node<T>  {
        let mut rng = rand::thread_rng();
        if let Some(values) = self.node_values.get(&node_type) {
            match node_type {
                NodeType::Input => {
                    let value = values[index % values.len()].clone();
                    return Node::new(index, node_type, value);
                },
                _ => {
                    let value = values.choose(&mut rng).unwrap();
                    return Node::new(index, node_type, value.new_instance());
                }
            }
        }
        
        Node::new(index, node_type, Ops::default())
    }

    pub fn regression(input_size: usize) -> NodeFactory<f32> {
        NodeFactory::new()
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
}