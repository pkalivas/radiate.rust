use crate::architects::nodes::node::Node;

use super::node_collection::NodeCollection;


pub struct Graph<T>
where
    T: Clone + PartialEq
{
    pub nodes: Vec<Node<T>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NodeCollection<Graph<T>, T> for Graph<T>
where
    T: Clone + PartialEq + Default
{
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn from_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph {
            nodes,
            _phantom: std::marker::PhantomData,
        }
    }

    fn get_node(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(index)
    }

    fn get_nodes(&self) -> &[Node<T>] {
        &self.nodes
    }

    fn get_nodes_mut(&mut self) -> &mut [Node<T>] {
        &mut self.nodes
    }
}

impl<T> Clone for Graph<T> 
where
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        Graph::from_nodes(self.nodes
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>())
    }
}

impl<T> Default for Graph<T> 
where
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Graph::new()
    }
}