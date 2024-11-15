use std::collections::VecDeque;

use super::super::node::Node;


pub struct GraphIterator<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub graph: &'a [Node<T>],
    pub completed: Vec<bool>,
    pub next_items: VecDeque<usize>,
    pub pending_index: usize,
}

impl<'a, T> GraphIterator<'a, T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: &'a [Node<T>]) -> Self {
        Self { 
            graph, 
            completed: vec![false; graph.len()],
            next_items: VecDeque::new(),
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

            let node = &self.graph[index];

            let mut degree = node.incoming.len();
            for incoming_index in &node.incoming {
                let incoming_node = &self.graph[*incoming_index];
                if self.completed[incoming_node.index] || incoming_node.is_recurrent() {
                    degree -= 1;
                }
            }

            if degree == 0 {
                self.completed[node.index] = true;
                self.next_items.push_back(node.index);
            } else {
                min_pending_index = std::cmp::min(min_pending_index, node.index);
            }
        }

        self.pending_index = min_pending_index;

        if let Some(index) = self.next_items.pop_front() {
            return Some(&self.graph[index]);
        } 

        None
    }
}
