use std::collections::HashSet;

use radiate_rust::engines::genome::genes::gene::{Gene, Valid};

use crate::architects::node_types::NodeType;

pub struct Node<T> 
where
    T: Clone + PartialEq 
{
    pub id: i32,
    pub node_type: NodeType,
    pub value: T,
    pub incoming: HashSet<i32>,
    pub outgoing: HashSet<i32>
}

impl<T> Node<T> 
where
    T: Clone + PartialEq 
{
    pub fn new(id: i32, node_type: NodeType, value: T) -> Node<T> {
        Node {
            id,
            node_type,
            value,
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }
}

impl<T> Valid for Node<T>
where
    T: Clone + PartialEq  {}

impl<T> Gene<Node<T>, T> for Node<T> 
where
    T: Clone + PartialEq 
{
    fn allele(&self) -> &T {
        &self.value
    }

    fn new_instance(&self) -> Node<T> {
        Node {
            id: self.id,
            node_type: self.node_type.clone(),
            value: self.value.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }

    fn from_allele(&self, allele: &T) -> Node<T> {
        Node {
            id: self.id,
            node_type: self.node_type.clone(),
            value: allele.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}

impl<T> Clone for Node<T> 
where
    T: Clone + PartialEq 
{
    fn clone(&self) -> Self {
        Node {
            id: self.id,
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
            && self.node_type == other.node_type 
            && self.value == other.value && self.incoming == other.incoming
            && self.outgoing == other.outgoing
    }
}

impl<T> std::fmt::Debug for Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Node {{ id: {}, node_type: {:?}, value: {:?}, incoming: {:?}, outgoing: {:?} }}",
               self.id, self.node_type, self.value, self.incoming, self.outgoing)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::architects::node_types::NodeType;

    #[test]
    fn test_new_node() {
        let node = Node::new(1, NodeType::Input, 0.5);
        assert_eq!(node.id, 1);
        assert_eq!(node.node_type, NodeType::Input);
        assert_eq!(node.value, 0.5);
        assert_eq!(node.incoming.len(), 0);
        assert_eq!(node.outgoing.len(), 0);
    }

    #[test]
    fn test_allele() {
        let node = Node::new(1, NodeType::Input, 0.5);
        assert_eq!(*node.allele(), 0.5);
    }

    #[test]
    fn test_new_instance() {
        let node = Node::new(1, NodeType::Input, 0.5);
        let new_node = node.new_instance();
        assert_eq!(new_node.id, 1);
        assert_eq!(new_node.node_type, NodeType::Input);
        assert_eq!(new_node.value, 0.5);
        assert_eq!(new_node.incoming.len(), 0);
        assert_eq!(new_node.outgoing.len(), 0);
    }

    #[test]
    fn test_from_allele() {
        let node = Node::new(1, NodeType::Input, 0.5);
        let new_node = node.from_allele(&0.6);
        assert_eq!(new_node.id, 1);
        assert_eq!(new_node.node_type, NodeType::Input);
        assert_eq!(new_node.value, 0.6);
        assert_eq!(new_node.incoming.len(), 0);
        assert_eq!(new_node.outgoing.len(), 0);
    }

    #[test]
    fn test_clone() {
        let node = Node::new(1, NodeType::Input, 0.5);
        let new_node = node.clone();
        assert_eq!(new_node.id, 1);
        assert_eq!(new_node.node_type, NodeType::Input);
        assert_eq!(new_node.value, 0.5);
        assert_eq!(new_node.incoming.len(), 0);
        assert_eq!(new_node.outgoing.len(), 0);
    }

    #[test]
    fn test_eq() {
        let node = Node::new(1, NodeType::Input, 0.5_f32);
        let new_node = node.clone();
        assert_eq!(node, new_node);
    }
}