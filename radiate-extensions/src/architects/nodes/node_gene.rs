use std::collections::HashSet;
use uuid::Uuid;
use radiate_rust::engines::genome::genes::gene::{Gene, Valid};
use crate::architects::node_types::NodeType;
use crate::architects::nodes::node::Node;

pub struct NodeGene<T>
where
    T: Clone + PartialEq
{
    pub id: Uuid,
    pub index: usize,
    pub node_type: NodeType,
    pub value: T,
    pub incoming: HashSet<usize>,
    pub outgoing: HashSet<usize>
}


impl<T> NodeGene<T> 
where
    T: Clone + PartialEq 
{
    pub fn new(index: usize, node_type: NodeType, value: T) -> NodeGene<T> {
        NodeGene {
            id: Uuid::new_v4(),
            index,
            node_type,
            value,
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }
}

impl<T> Node<NodeGene<T>, T> for NodeGene<T>
where
    T: Clone + PartialEq + Default
{
    fn new_node(index: usize, node_type: NodeType, value: T) -> NodeGene<T> {
        NodeGene::new(index, node_type, value)
    }

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn index(&self) -> &usize {
        &self.index
    }

    fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn incoming_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.incoming
    }

    fn outgoing_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.outgoing
    }
}

impl<T> Valid for NodeGene<T>
where
    T: Clone + PartialEq  {}

impl<T> Default for NodeGene<T>
where
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        NodeGene {
            id: Uuid::new_v4(),
            index: 0,
            node_type: NodeType::Input,
            value: Default::default(),
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }
}

impl<T> Gene<NodeGene<T>, T> for NodeGene<T>
where
    T: Clone + PartialEq 
{
    fn allele(&self) -> &T {
        &self.value
    }

    fn new_instance(&self) -> NodeGene<T> {
        NodeGene {
            id: Uuid::new_v4(),
            index: self.index,
            node_type: self.node_type.clone(),
            value: self.value.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }

    fn from_allele(&self, allele: &T) -> NodeGene<T> {
        NodeGene {
            id: Uuid::new_v4(),
            index: self.index,
            node_type: self.node_type.clone(),
            value: allele.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}

impl<T> Clone for NodeGene<T>
where
    T: Clone + PartialEq,
{
    fn clone(&self) -> Self {
        NodeGene {
            id: self.id,
            index: self.index,
            node_type: self.node_type.clone(),
            value: self.value.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}

impl<T> PartialEq for NodeGene<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.id == other.id
            && self.node_type == other.node_type
            && self.value == other.value
            && self.incoming == other.incoming
            && self.outgoing == other.outgoing
    }
}
