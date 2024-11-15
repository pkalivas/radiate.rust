use radiate_rust::engines::genome::genes::gene::{Gene, Valid};
use std::collections::HashSet;
use uuid::Uuid;

use crate::architects::schema::{direction::Direction, node_types::NodeType};
use crate::operations::op::Ops;

pub struct Node<T>
where
    T: Clone + PartialEq,
{
    pub id: Uuid,
    pub index: usize,
    pub value: Ops<T>,
    pub arity: Option<u8>,
    pub enabled: bool,
    pub node_type: NodeType,
    pub direction: Direction,
    pub incoming: HashSet<usize>,
    pub outgoing: HashSet<usize>,
}

impl<T> Node<T>
where
    T: Clone + PartialEq,
{
    pub fn new(index: usize, node_type: NodeType, value: Ops<T>) -> Self {
        Self {
            id: Uuid::new_v4(),
            index,
            value,
            arity: None,
            enabled: true,
            direction: Direction::Forward,
            node_type,
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
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
        self.direction == Direction::Backward
            || self.incoming.contains(&self.index)
            || self.outgoing.contains(&self.index)
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
}

impl<T> Gene<Node<T>, Ops<T>> for Node<T>
where
    T: Clone + PartialEq + Default,
{
    fn allele(&self) -> &Ops<T> {
        &self.value
    }

    fn new_instance(&self) -> Node<T> {
        Node {
            id: Uuid::new_v4(),
            index: self.index,
            arity: self.arity.clone(),
            enabled: self.enabled,
            value: self.value.new_instance(),
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
        }
    }

    fn from_allele(&self, allele: &Ops<T>) -> Node<T> {
        Node {
            id: Uuid::new_v4(),
            index: self.index,
            arity: self.arity.clone(),
            value: allele.clone(),
            enabled: self.enabled,
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
        }
    }
}

impl<T> Valid for Node<T>
where
    T: Clone + PartialEq,
{
    fn is_valid(&self) -> bool {
        match self.node_type {
            NodeType::Input => self.incoming.is_empty() && !self.outgoing.is_empty(),
            NodeType::Output => self.incoming.len() > 0,
            NodeType::Gate => self.incoming.len() == self.arity().unwrap() as usize,
            NodeType::Aggregate => !self.incoming.is_empty() && !self.outgoing.is_empty(),
            NodeType::Weight => self.incoming.len() == 1 && self.outgoing.len() == 1,
            NodeType::Link => self.incoming.len() == 1 && self.outgoing.len() > 0,
        }
    }
}

impl<T> Clone for Node<T>
where
    T: Clone + PartialEq,
{
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            index: self.index.clone(),
            arity: self.arity.clone(),
            enabled: self.enabled,
            value: self.value.clone(),
            direction: self.direction.clone(),
            node_type: self.node_type.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
        }
    }
}

impl<T> PartialEq for Node<T>
where
    T: Clone + PartialEq,
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

impl<T> Default for Node<T>
where
    T: Clone + PartialEq + Default,
{
    fn default() -> Self {
        Node {
            id: Uuid::new_v4(),
            index: 0,
            arity: None,
            enabled: true,
            value: Ops::default(),
            direction: Direction::Forward,
            node_type: NodeType::Input,
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
        }
    }
}

impl<T> std::fmt::Display for Node<T>
where
    T: Clone + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl<T> std::fmt::Debug for Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let incoming = self
            .incoming
            .iter()
            .map(|idx| idx.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(
            f,
            "[{:<3}] {:>10?} :: {:<12} E: {:<5} V:{:<5} R:{:<5} {:<2} {:<2} < [{}]",
            self.index,
            format!("{:?}", self.node_type)[..3].to_owned(),
            format!("{:?}", self.value).to_owned(),
            self.is_valid(),
            self.enabled,
            self.is_recurrent(),
            self.incoming.len(),
            self.outgoing.len(),
            incoming
        )
    }
}
