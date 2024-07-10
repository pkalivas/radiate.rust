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
    pub index: i32,
    pub node_type: NodeType,
    pub value: T,
    pub incoming: HashSet<i32>,
    pub outgoing: HashSet<i32>
}


impl<T> NodeGene<T> 
where
    T: Clone + PartialEq 
{
    pub fn new(index: i32, node_type: NodeType, value: T) -> NodeGene<T> {
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

impl<T> Node<T> for NodeGene<T>
where
    T: Clone + PartialEq
{
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn index(&self) -> &i32 {
        &self.index
    }

    fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn incoming(&mut self) -> &mut HashSet<i32> {
        &mut self.incoming
    }

    fn outgoing(&mut self) -> &mut HashSet<i32> {
        &mut self.outgoing
    }
}

impl<T> Valid for NodeGene<T>
where
    T: Clone + PartialEq  {}

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
