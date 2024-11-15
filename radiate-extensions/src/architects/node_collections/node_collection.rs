
use std::collections::{HashSet, VecDeque};

use radiate_rust::engines::genome::genes::gene::Valid;

use crate::architects::node_collections::node::Node;

pub trait NodeCollection<C, T>: Valid
where
    C: NodeCollection<C, T> + Default + Clone,
    T: Clone + PartialEq + Default
{
    fn from_nodes(nodes: Vec<Node<T>>) -> Self;

    fn get(&self, index: usize) -> Option<&Node<T>>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>>;
    
    fn get_nodes(&self) -> &[Node<T>];
    fn get_nodes_mut(&mut self) -> &mut [Node<T>];

    fn set_cycles(self, indecies: Vec<usize>) -> C;

    fn add(&mut self, nodes: Vec<Node<T>>);

    fn set(&mut self, index: usize, node: Node<T>) -> &mut Self {
        self.get_nodes_mut()[index] = node;
        self
    }

    fn iter(&self) -> std::slice::Iter<Node<T>> {
        self.get_nodes().iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<Node<T>> {
        self.get_nodes_mut().iter_mut()
    }
    
    fn len(&self) -> usize {
        self.get_nodes().len()
    }

    fn attach(&mut self, incoming: usize, outgoing: usize) -> &mut Self {
        self.get_nodes_mut()[incoming].outgoing_mut().insert(outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().insert(incoming);
        self
    }

    fn detach(&mut self, incoming: usize, outgoing: usize) -> &mut Self {
        self.get_nodes_mut()[incoming].outgoing_mut().remove(&outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().remove(&incoming);
        self
    }

    fn reindex(&self, index: usize) -> C {
        let mut new_nodes = self.get_nodes()
            .iter()
            .enumerate()
            .map(|(i, node)| Node {
                index: index + i,
                incoming: HashSet::new(),
                outgoing: HashSet::new(),
                ..node.clone()
            })
            .collect::<Vec<Node<T>>>();

        let ref_new_nodes = new_nodes.clone();

        let old_nodes = self.get_nodes()
            .iter()
            .enumerate()
            .map(|(i, node)| (node.index, i))
            .collect::<std::collections::BTreeMap<usize, usize>>();

        for i in 0..new_nodes.len() {
            let old_node = self.get(i).unwrap();
            let new_node = &mut new_nodes[i];

            for incoming in old_node.incoming.iter() {
                if let Some(old_index) = old_nodes.get(incoming) {
                    // let old_incoming = self.get(*old_index).unwrap();
                    new_node.incoming_mut().insert(ref_new_nodes[*old_index].index);
                }
            }

            for outgoing in old_node.outgoing.iter() {
                if let Some(old_index) = old_nodes.get(outgoing) {
                    // let old_outgoing = self.get(*old_index).unwrap();
                    new_node.outgoing_mut().insert(ref_new_nodes[*old_index].index);
                }
            }
        }

        C::from_nodes(new_nodes)
    }
}


pub fn get_cycles<T>(nodes: &[Node<T>], index: usize) -> Vec<usize>
where
    T: Clone + PartialEq + Default
{
    let mut path = Vec::new();
    let mut seen = HashSet::new();
    let mut current = nodes[index].incoming()
        .iter()
        .cloned()
        .collect::<VecDeque<usize>>();

    while current.len() > 0 {
        let current_index = current.pop_front().unwrap();
        let current_node = &nodes[current_index];

        if seen.contains(&current_index) {
            continue;
        }

        if current_index == index {
            return path;
        }

        seen.insert(current_index);

        if current_node.incoming().len() != 0 {
            path.push(current_index);
            for outgoing in current_node.incoming().iter() {
                current.push_back(*outgoing);
            }
        }
    }

    Vec::new()
}