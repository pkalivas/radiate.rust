use core::panic;
use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::architects::node_collections::node::Node;
use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::schema::node_types::NodeType;

pub trait Modifier<C, T>
where
    C: NodeCollection<C, T> + Default + Clone,
    T: Clone + PartialEq + Default
{
    fn modify(&self, collection: &mut C) -> C;
}


pub fn can_connect<C, T>(collection: &C, source: usize, target: usize) -> bool
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    let source_node = &collection.get(source).unwrap();
    let target_node = &collection.get(target).unwrap();

    if source_node.outgoing.len() == 0 || source_node.is_recurrent() {
        return false;
    }

    let would_create_cycle = would_create_cycle(collection, source, target);
    let nodes_are_weights = source_node.node_type == NodeType::Weight || target_node.node_type == NodeType::Weight;

    return !would_create_cycle && !nodes_are_weights && source != target;
}

pub fn would_create_cycle<C, T>(collection: &C, source: usize, target: usize) -> bool
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    let mut seen = HashSet::new();
    let mut visited = collection.get(target).unwrap().outgoing.iter().collect::<Vec<&usize>>();

    while visited.len() != 0 {
        let node_index = visited.pop().unwrap();

        seen.insert(*node_index);

        if *node_index == source {
            return true;
        }

        for edge_index in collection.get(*node_index).unwrap().outgoing.iter().filter(|edge_index| !seen.contains(edge_index)) {
            visited.push(edge_index);
        }
    }

    return false;
}

pub fn is_locked<T>(node: &Node<T>) -> bool
where
    T: Clone + PartialEq + Default
{
    if node.node_type == NodeType::Aggregate || node.node_type == NodeType::Output {
        return false;
    }

    match node.arity() {
        Some(arity) => return node.incoming.len() == *arity as usize,
        None => panic!("Node arity must be set before checking if it is locked.")
    }   
}

pub fn random_source_node<C, T>(collection: &C) -> &Node<T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    random_node_of_type(collection, vec![
        NodeType::Input, 
        NodeType::Gate,
        NodeType::Aggregate,
        NodeType::Link
    ])
}

pub fn random_target_node<C, T>(collection: &C) -> &Node<T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    random_node_of_type(collection, vec![
        NodeType::Output,
        NodeType::Aggregate
    ])
}

fn random_node_of_type<C, T>(collection: &C, node_types: Vec<NodeType>) -> &Node<T>
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    if node_types.len() == 0 {
        panic!("At least one node type must be specified.");
    }

    let mut random = rand::thread_rng();
    let gene_node_type = node_types.choose(&mut random).unwrap();

    let genes = match gene_node_type {
        NodeType::Input => collection.iter().filter(|node| node.node_type == NodeType::Input).collect::<Vec<&Node<T>>>(),
        NodeType::Weight => collection.iter().filter(|node| node.node_type == NodeType::Weight).collect::<Vec<&Node<T>>>(),
        NodeType::Gate => collection.iter().filter(|node| node.node_type == NodeType::Gate).collect::<Vec<&Node<T>>>(),
        NodeType::Output => collection.iter().filter(|node| node.node_type == NodeType::Output).collect::<Vec<&Node<T>>>(),
        NodeType::Link => collection.iter().filter(|node| node.node_type == NodeType::Link).collect::<Vec<&Node<T>>>(),
        NodeType::Aggregate => collection.iter().filter(|node| node.node_type == NodeType::Aggregate).collect::<Vec<&Node<T>>>(),
    };

    if genes.len() == 0 {
        return random_node_of_type(collection, node_types.iter().filter(|nt| *nt != gene_node_type).cloned().collect());
    }

    return genes.choose(&mut random).unwrap()
}