use std::collections::VecDeque;

use crate::NodeCollection;

use super::super::node::Node;
use super::super::graph::Graph;

pub struct GraphIterator<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub graph: &'a Graph<T>,
    pub completed: Vec<bool>,
    pub index_queue: VecDeque<usize>,
    pub pending_index: usize,
}

impl<'a, T> GraphIterator<'a, T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: &'a Graph<T>) -> Self {
        Self { 
            graph, 
            completed: vec![false; graph.len()],
            index_queue: VecDeque::new(),
            pending_index: 0,
        }
    }
}

impl<'a, T> Iterator for GraphIterator<'a, T> 
where
    T: Clone + PartialEq + Default
{
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut min_pending_index = self.graph.len();
        for index in self.pending_index..self.graph.len() {
            if self.completed[index] {
                continue;
            }

            if let Some(node) = self.graph.get(index) {
                let mut degree = node.incoming.len();
                for incoming_index in &node.incoming {
                    if let Some(incoming_node) = self.graph.get(*incoming_index) {
                        if self.completed[incoming_node.index] || incoming_node.is_recurrent() {
                            degree -= 1;
                        }
                    }
                }
    
                if degree == 0 {
                    self.completed[node.index] = true;
                    self.index_queue.push_back(node.index);
                } else {
                    min_pending_index = std::cmp::min(min_pending_index, node.index);
                }
            }
        }

        self.pending_index = min_pending_index;

        if let Some(index) = self.index_queue.pop_front() {
            return match self.graph.get(index) {
                Some(node) => Some(node),
                None => None,
            }
        } 

        None
    }
}
