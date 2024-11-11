use crate::architects::nodes::tracer::Tracer;
use crate::architects::schema::node_types::NodeType;

use super::graph::Graph;
use super::node_collection::NodeCollection;


const CHECKS_WITHOUT_PROGRESS: i32 = 5000;


pub struct GraphReducer<T>
where
    T: Clone + PartialEq + Default
{
    pub graph: Graph<T>,
    pub tracers: Vec<Tracer<T>>,
}

impl<T> GraphReducer<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: Graph<T>) -> GraphReducer<T> {
        let tracers = graph
            .iter()
            .map(|node| Tracer::new(node.input_size()))
            .collect::<Vec<Tracer<T>>>();

        GraphReducer { graph, tracers }
    }

    pub fn reduce(&mut self, inputs: &[T]) -> Vec<T> {
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
                        if completed[*incoming] || self.graph.get(*incoming).unwrap().is_recurrent() {
                            degree -= 1;
                        }
                    }

                    if degree == 0 {
                        if node.node_type == NodeType::Input {
                            self.tracers[node.index].add_input(inputs[node.index].clone());
                        } else {
                            for incoming in &node.incoming {
                                let arg = self.tracers[*incoming].result.clone().unwrap_or_else(|| T::default());
                                self.tracers[node.index].add_input(arg);
                            }
                        }
            
                        completed[node.index] = true;
                        self.tracers[node.index].activate(&node);

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
}
