use std::collections::HashSet;
use radiate_rust::engines::genome::genes::gene::{Gene, Valid};
use uuid::Uuid;
use crate::architects::schema::node_types::NodeType;

// pub trait Node<N, T> 
// where 
//     N: Node<N, T> + Clone + Default,
//     T: Clone + PartialEq + Default
// {
//     fn new_node(index: usize, node_type: NodeType, value: T) -> N;
//     fn id(&self) -> &Uuid;
//     fn index(&self) -> &usize;
//     fn node_type(&self) -> &NodeType;
//     fn value(&self) -> &T;
//     fn is_recurrent(&self) -> bool;
//     fn incoming_mut(&mut self) -> &mut HashSet<usize>;
//     fn outgoing_mut(&mut self) -> &mut HashSet<usize>;
//     fn incoming(&self) -> &HashSet<usize>;
//     fn outgoing(&self) -> &HashSet<usize>;
// }

pub struct Node<T>
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

impl<T> Node<T> 
where
    T: Clone + PartialEq 
{
    pub fn new(index: usize, node_type: NodeType, value: T) -> Node<T> {
        Node {
            id: Uuid::new_v4(),
            index,
            node_type,
            value,
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn is_recurrent(&self) -> bool {
        self.incoming.contains(&self.index)
    }

    pub fn incoming(&self) -> &HashSet<usize> {
        &self.incoming
    }

    pub fn outgoing(&self) -> &HashSet<usize> {
        &self.outgoing
    }

    pub fn incoming_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.incoming
    }

    pub fn outgoing_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.outgoing
    }
}

impl<T> Gene<Node<T>, T> for Node<T>
where
    T: Clone + PartialEq
{
    fn allele(&self) -> &T {
        &self.value
    }

    fn new_instance(&self) -> Node<T> {
        Node {
            id: Uuid::new_v4(),
            index: self.index,
            node_type: self.node_type.clone(),
            value: self.value.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }

    fn from_allele(&self, allele: &T) -> Node<T> {
        Node {
            id: Uuid::new_v4(),
            index: self.index,
            node_type: self.node_type.clone(),
            value: allele.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}


impl<T> Valid for Node<T>
where
    T: Clone + PartialEq  {}


impl<T> Clone for Node<T>
where
    T: Clone + PartialEq
{
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            index: self.index.clone(),
            node_type: self.node_type.clone(),
            value: self.value.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}


impl<T> PartialEq for Node<T>
where
    T: Clone + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.index == other.index 
            && self.node_type == other.node_type 
            && self.value == other.value 
            && self.incoming == other.incoming 
            && self.outgoing == other.outgoing
    }
}


impl<T> Default for Node<T>
where
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Node {
            id: Uuid::new_v4(),
            index: 0,
            node_type: NodeType::Input,
            value: T::default(),
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }
}


impl<T> std::fmt::Display for Node<T>
where
    T: Clone + PartialEq
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}


impl<T> std::fmt::Debug for Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {}, index: {}, node_type: {:?}, value: {:?}, incoming: {:?}, outgoing: {:?} }}", self.id, self.index, self.node_type, self.value, self.incoming, self.outgoing)
    }
}