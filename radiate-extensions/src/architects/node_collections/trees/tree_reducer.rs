use std::collections::VecDeque;

use crate::Node;
use crate::Tracer;
use crate::NodeType;

use super::tree::Tree;
use super::super::node_collection::NodeCollection;


const CHECKS_WITHOUT_PROGRESS: i32 = 5000;


pub struct TreeReducer<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub tree: &'a Tree<T>,
    pub tracers: Vec<Tracer<T>>,
    pub order: Vec<usize>,
}

impl<'a, T> TreeReducer<'a, T>
where 
    T: Clone + PartialEq + Default
{   
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self { 
            tree, 
            tracers: tree
                .iter()
                .map(|node| Tracer::new(TreeReducer::input_size(node)))
                .collect::<Vec<Tracer<T>>>(), 
            order: Vec::with_capacity(tree.len())
        }
    }

    #[inline]
    pub fn reduce(&mut self, inputs: &[T]) -> Vec<T> {
        let mut checks = 0;
        let mut completed = vec![false; self.tree.len()];
        let mut result = Vec::new();
        let mut stack = self.tree
            .iter()
            .filter(|node| node.incoming.is_empty())
            .map(|node| node.index)
            .collect::<VecDeque<usize>>();

        while stack.len() > 0 {
            if checks > CHECKS_WITHOUT_PROGRESS {
                panic!("Failed to reduce tree.");
            }

            if let Some(node_index) = stack.back() {
                if let Some(node) = self.tree.get(*node_index) {
                    let mut degree = node.outgoing.len();
                    for outgoing in &node.outgoing {
                        if let Some(outgoing_node) = self.tree.get(*outgoing) {
                            if completed[outgoing_node.index] {
                                degree -= 1;
                            }
                        }
                    }

                    if degree == 0 {

                        if node.node_type == NodeType::Leaf {
                            let temp = node.value.apply(inputs);
                            self.tracers[node.index].add_input(temp.clone());
                        } else {
                            for outgoing in &node.outgoing {
                                let arg = self.tracers[*outgoing].result.clone().unwrap_or_else(|| T::default());
                                self.tracers[node.index].add_input(arg);
                            }
                        }

                        completed[node.index] = true;
                        self.order.push(node.index);
                        self.tracers[node.index].eval(node);
                        stack.pop_back();

                        if node.node_type == NodeType::Root {
                            result.push(self.tracers[node.index].result.clone().unwrap());
                        }
                    } else {
                        for outgoing in &node.outgoing {
                            if !completed[*outgoing] {
                                stack.push_back(*outgoing);
                                break;
                            }
                        }

                        checks += 1;
                    }
                }
            }
        }

        result
    }

    fn input_size(node: &Node<T>) -> usize {
        match node.node_type {
            NodeType::Input | NodeType::Link | NodeType::Leaf => 1,
            NodeType::Gate => node.value.arity() as usize,
            _ => node.outgoing.len()
        }
    }
}