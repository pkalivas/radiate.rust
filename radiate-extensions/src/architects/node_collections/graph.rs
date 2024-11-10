use crate::architects::nodes::node::Node;

use super::node_collection::NodeCollection;


pub struct Graph<N, T> {
    pub nodes: Vec<N>,
    _phantom_t: std::marker::PhantomData<T>,
}

impl<N, T> Graph<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }
}

impl<N, T> NodeCollection<Graph<N, T>, N, T> for Graph<N, T>
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }

    fn from_nodes(nodes: Vec<N>) -> Self {
        Graph {
            nodes,
            _phantom_t: std::marker::PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn get_nodes(&self) -> &[N] {
        &self.nodes
    }

    fn get_node(&self, index: usize) -> Option<&N> {
        self.nodes.get(index)
    }

    fn get_node_mut(&mut self, index: usize) -> Option<&mut N> {
        self.nodes.get_mut(index)
    }

    fn get_nodes_mut(&mut self) -> &mut [N] {
        &mut self.nodes
    }

    fn reindex(&self, index: usize) -> Graph<N, T> {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(N::new_node(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Graph::from_nodes(new_nodes)
    }

    fn set_cycles(&mut self) -> Graph<N, T> {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            let mut new_node = N::new_node(*node.index(), node.node_type().clone(), node.value().clone());
            for incoming in node.incoming().iter() {
                new_node.incoming_mut().insert(*incoming);
            }
            for outgoing in node.outgoing().iter() {
                new_node.outgoing_mut().insert(*outgoing);
            }
            new_nodes.push(new_node);
        }

        Graph::from_nodes(new_nodes)
    }
}

impl<N, T> Clone for Graph<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(N::new_node(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Graph::from_nodes(new_nodes)
    }
}

impl<N, T> Default for Graph<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Graph::new()
    }
}