use crate::architects::nodes::node::Node;

use super::node_collection::NodeCollection;


pub struct Graph<T>
where
    T: Clone + PartialEq
{
    pub nodes: Vec<Node<T>>,
    _phantom_t: std::marker::PhantomData<T>,
}

impl<T> Graph<T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }
}

impl<T> NodeCollection<Graph<T>, T> for Graph<T>
where
    T: Clone + PartialEq + Default
{
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }

    fn from_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph {
            nodes,
            _phantom_t: std::marker::PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn get_nodes(&self) -> &[Node<T>] {
        &self.nodes
    }

    fn get_node(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(index)
    }

    fn get_nodes_mut(&mut self) -> &mut [Node<T>] {
        &mut self.nodes
    }

    fn reindex(&self, index: usize) -> Graph<T> {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(Node::new(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Graph::from_nodes(new_nodes)
    }

    fn set_cycles(&mut self) -> Graph<T> {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            let mut new_node = Node::new(*node.index(), node.node_type().clone(), node.value().clone());
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

impl<T> Clone for Graph<T> 
where
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(Node::new(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Graph::from_nodes(new_nodes)
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