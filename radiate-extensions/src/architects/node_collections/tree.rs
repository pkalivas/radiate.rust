use crate::architects::nodes::node::Node;

use super::node_collection::NodeCollection;


pub struct Tree<N, T> {
    pub nodes: Vec<N>,
    _phantom_t: std::marker::PhantomData<T>,
}

impl<N, T> Tree<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }
}

impl<N, T> NodeCollection<Tree<N, T>, N, T> for Tree<N, T>
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            _phantom_t: std::marker::PhantomData,
        }
    }

    fn from_nodes(nodes: Vec<N>) -> Self {
        Tree {
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

    fn reindex(&self, index: usize) -> Tree<N, T> {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(N::new_node(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Tree::from_nodes(new_nodes)
    }

    fn set_cycles(&mut self) -> Tree<N, T> {
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

        Tree::from_nodes(new_nodes)
    }
}

impl<N, T> Clone for Tree<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        let mut new_nodes = Vec::new();
        for node in self.nodes.iter() {
            new_nodes.push(N::new_node(*node.index(), node.node_type().clone(), node.value().clone()));
        }

        Tree::from_nodes(new_nodes)
    }
}

impl<N, T> Default for Tree<N, T> 
where
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Tree::new()
    }
}