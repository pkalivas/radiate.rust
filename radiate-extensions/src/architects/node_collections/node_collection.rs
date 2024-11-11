
use std::collections::{HashSet, VecDeque};

use crate::architects::{nodes::node::Node, schema::direction::Direction};

pub trait NodeCollection<C, T>
where
    C: NodeCollection<C, T> + Default + Clone,
    T: Clone + PartialEq + Default
{
    fn from_nodes(nodes: Vec<Node<T>>) -> Self;

    fn get(&self, index: usize) -> Option<&Node<T>>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>>;
    
    fn get_nodes(&self) -> &[Node<T>];
    fn get_nodes_mut(&mut self) -> &mut [Node<T>];

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

    fn attach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().insert(outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().insert(incoming);
    }

    fn detach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().remove(&outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().remove(&incoming);
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
                    let old_incoming = self.get(*old_index).unwrap();
                    new_node.incoming_mut().insert(old_incoming.index);
                }
            }

            for outgoing in old_node.outgoing.iter() {
                if let Some(old_index) = old_nodes.get(outgoing) {
                    let old_outgoing = self.get(*old_index).unwrap();
                    new_node.outgoing_mut().insert(old_outgoing.index);
                }
            }
        }

        C::from_nodes(new_nodes)
    }

    fn set_cycles(&mut self, indecies: Vec<usize>) -> &mut Self {
        if indecies.len() == 0 {
            let all_indices = self.get_nodes()
                .iter()
                .map(|node| node.index)
                .collect::<Vec<usize>>();

            return self.set_cycles(all_indices)
        }

        for idx in indecies {
            let node_cycles = get_cycles(self.get_nodes(), idx);

            if node_cycles.len() == 0 {
                let node = self.get_mut(idx).unwrap();
                (*node).direction = Direction::Forward;
            } else {
                for cycle_idx in node_cycles {
                    let node = self.get_mut(cycle_idx).unwrap();
                    (*node).direction = Direction::Backward;
                }
            }
        }

        self
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

