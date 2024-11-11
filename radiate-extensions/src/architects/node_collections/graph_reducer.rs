use crate::architects::nodes::tracer::Tracer;

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
            .map(|_| Tracer::new(1))
            .collect::<Vec<Tracer<T>>>();

        GraphReducer { graph, tracers }
    }

    pub fn reduce(_: &[T]) -> Vec<T> {
        unimplemented!("GraphReducer::reduce");
    }
}