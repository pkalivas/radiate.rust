use crate::architects::node_collections::node::Node;

use super::super::node_collection::NodeCollection;


pub struct Graph<T>
where
    T: Clone + PartialEq
{
    pub nodes: Vec<Node<T>>,
}

impl<T> NodeCollection<Graph<T>, T> for Graph<T>
where
    T: Clone + PartialEq + Default
{
    fn from_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph { nodes }
    }

    fn get(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(index)
    }

    fn get_nodes(&self) -> &[Node<T>] {
        &self.nodes
    }

    fn get_nodes_mut(&mut self) -> &mut [Node<T>] {
        &mut self.nodes
    }

    fn add(&mut self, node: Node<T>) -> &mut Self {
        self.nodes.push(node);
        self
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
        Graph {
            nodes: Vec::new(),
        }
    }
}