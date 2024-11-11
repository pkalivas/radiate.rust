use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;
use radiate_rust::engines::genome::genes::gene::{Gene, Valid};

use crate::{architects::{factories::node_factory::NodeFactory, schema::{direction::Direction, node_types::NodeType}}, operations::op::Ops};

pub struct Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub id: Uuid,
    pub index: usize,
    pub arity: Option<u8>,
    pub value: Ops<T>,
    pub node_type: NodeType,
    pub direction: Direction,
    pub factory: Option<&'a NodeFactory<T>>,
    pub incoming: HashSet<usize>,
    pub outgoing: HashSet<usize>
}

impl<'a, T> Node<'a, T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new(index: usize, node_type: NodeType, value: Ops<T>) -> Node<'a, T> {
        Node {
            id: Uuid::new_v4(),
            index,
            arity: None,
            value,
            direction: Direction::Forward,
            node_type,
            factory: None,
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }

    pub fn from_value(index: usize, node_type: NodeType, value: T) -> Node<'a, T> {
        Node {
            id: Uuid::new_v4(),
            index,
            arity: None,
            value: Ops::Value(value),
            direction: Direction::Forward,
            node_type,
            factory: None,
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

    pub fn arity(&self) -> &Option<u8> {
        &self.arity
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn value(&self) -> &Ops<T> {
        &self.value
    }

    pub fn is_recurrent(&self) -> bool {
        self.incoming.contains(&self.index) 
            || self.outgoing.contains(&self.index) 
            || self.direction == Direction::Backward
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

    pub fn set_arity(mut self, arity: u8) -> Self {
        self.arity = Some(arity);
        self
    }

    pub fn set_factory(mut self, factory: &'a NodeFactory<T>) -> Self {
        self.factory = Some(factory);
        self
    }
}

impl<'a, T> Gene<Node<'a, T>, Ops<T>> for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn allele(&self) -> &Ops<T> {
        &self.value
    }

    fn new_instance(&self) -> Node<'a, T> {
        if let Some(factory) = &self.factory {
            let temp_node = factory.new_node(self.index, self.node_type.clone());
            // TODO: need to fix this the arity could be off. Can't think of a clean solution right now.
            return Node {
                id: Uuid::new_v4(),
                index: self.index,
                arity: self.arity.clone(),
                value: Ops::default(),
                // value: temp_node.value.clone(),
                direction: self.direction.clone(),
                node_type: self.node_type.clone(),
                factory: match &self.factory {
                    Some(f) => Some(f),
                    None => None
                },
                incoming: self.incoming.clone(),
                outgoing: self.outgoing.clone()
            }
        }

        Node {
            id: Uuid::new_v4(),
            index: self.index,
            arity: self.arity.clone(),
            value: self.value.clone(),
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            factory: match &self.factory {
                Some(f) => Some(f),
                None => None
            },
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }

    fn from_allele(&self, allele: &Ops<T>) -> Node<'a, T> {
        Node {
            id: Uuid::new_v4(),
            index: self.index,
            arity: self.arity.clone(),
            value: allele.clone(),
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            factory: match &self.factory {
                Some(f) => Some(f),
                None => None
            },
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}


impl<'a, T> Valid for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn is_valid(&self) -> bool {
        match self.node_type {
            NodeType::Input => self.incoming.is_empty() && !self.outgoing.is_empty(),
            NodeType::Output => self.incoming.len() > 0,
            NodeType::Gate => !self.incoming.is_empty() && !self.outgoing.is_empty(),
            NodeType::Aggregate => !self.incoming.is_empty() && !self.outgoing.is_empty(),
            NodeType::Weight => self.incoming.len() == 1 && self.outgoing.len() == 1,
            NodeType::Link => self.incoming.len() == 1 && self.outgoing.len() > 0
        }
    }
}


impl<'a, T> Clone for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            index: self.index.clone(),
            arity: self.arity.clone(),
            value: self.value.clone(),
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            factory: match &self.factory {
                Some(f) => Some(f),
                None => None
            },
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone()
        }
    }
}


impl<'a, T> PartialEq for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.index == other.index
            && self.arity == other.arity
            && self.value == other.value
            && self.direction == other.direction
            && self.node_type == other.node_type 
            && self.incoming == other.incoming 
            && self.outgoing == other.outgoing
    }
}


impl<'a, T> Default for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Node {
            id: Uuid::new_v4(),
            index: 0,
            arity: None,
            value: Ops::default(),
            direction: Direction::Forward,
            node_type: NodeType::Input,
            factory: None,
            incoming: HashSet::new(),
            outgoing: HashSet::new()
        }
    }
}


impl<'a, T> std::fmt::Display for Node<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}


impl<'a, T> std::fmt::Debug for Node<'a, T>
where
    T: Clone + PartialEq + std::fmt::Debug + Default
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {}, index: {}, dir: {:?}, node_type: {:?}, arity: {:?}, value: {:?}, incoming: {:?}, outgoing: {:?} }}", 
            self.id,
            self.index,
            self.direction,
            self.node_type,
            self.arity,
            self.value, 
            self.incoming, 
            self.outgoing)
    }
}