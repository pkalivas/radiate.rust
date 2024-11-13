use crate::architects::node_collections::tracer::Tracer;
use crate::architects::schema::node_types::NodeType;

use super::graph::Graph;
use super::super::node_collection::NodeCollection;


const CHECKS_WITHOUT_PROGRESS: i32 = 5000;


pub struct GraphReducer<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub graph: &'a Graph<T>,
    pub tracers: Vec<Tracer<T>>,
    pub order: Vec<usize>,
}

impl<'a, T> GraphReducer<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: &'a Graph<T>) -> GraphReducer<'a, T> {
        GraphReducer { 
            graph, 
            tracers: graph
                .iter()
                .map(|node| Tracer::new(node.input_size()))
                .collect::<Vec<Tracer<T>>>(), 
            order: Vec::with_capacity(graph.len())
        }
    }

    pub fn reduce(&mut self, inputs: &[T]) -> Vec<T> {
        if !self.order.is_empty() {
            return self.reduce_with_order(inputs);
        } 

        let mut checks = 0;
        let mut completed = vec![false; self.graph.len()];
        let mut result = Vec::new();

        let mut pending_index = 0;
        while pending_index < self.graph.len() {
            if checks > CHECKS_WITHOUT_PROGRESS {
                panic!("Failed to reduce graph.");
            }

            let mut min_pending_index = self.graph.len();
            for index in pending_index..self.graph.len() {
                if let Some(node) = self.graph.get(index) {
                    if completed[node.index] {
                        continue;
                    }

                    let mut degree = node.incoming.len();
                    for incoming in &node.incoming {
                        if let Some(incoming_node) = self.graph.get(*incoming) {
                            if completed[incoming_node.index] || incoming_node.is_recurrent() {
                                degree -= 1;
                            }
                        }
                    }

                    if degree == 0 {
                        self.order.push(node.index);
                        if node.node_type == NodeType::Input {
                            self.tracers[node.index].add_input(inputs[node.index].clone());
                        } else {
                            for incoming in &node.incoming {
                                let arg = self.tracers[*incoming].result.clone().unwrap_or_else(|| T::default());
                                self.tracers[node.index].add_input(arg);
                            }
                        }
            
                        completed[node.index] = true;
                        self.tracers[node.index].eval(&node);

                        if node.node_type == NodeType::Output {
                            result.push(self.tracers[node.index].result.clone().unwrap());
                        }
                    } else {
                        min_pending_index = std::cmp::min(min_pending_index, node.index);
                    }
                }
            }

            pending_index = min_pending_index;
            checks = if min_pending_index == pending_index { checks + 1 } else { 0 };
        }

        result
    }

    fn reduce_with_order(&mut self, inputs: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        for index in &self.order {
            let node = self.graph.get(*index).unwrap();
            if node.node_type == NodeType::Input {
                self.tracers[node.index].add_input(inputs[node.index].clone());
            } else {
                for incoming in &node.incoming {
                    let arg = self.tracers[*incoming].result.clone().unwrap_or_else(|| T::default());
                    self.tracers[node.index].add_input(arg);
                }
            }

            self.tracers[node.index].eval(&node);

            if node.node_type == NodeType::Output {
                result.push(self.tracers[node.index].result.clone().unwrap());
            }
        }

        result
    }
}
