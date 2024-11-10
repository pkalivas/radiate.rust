
use crate::architects::nodes::node::Node;

pub trait NodeCollection<C, T>
where
    C: NodeCollection<C, T> + Default,
    T: Clone + PartialEq + Default
{
    fn new() -> Self;
    fn from_nodes(nodes: Vec<Node<T>>) -> Self;
    fn len(&self) -> usize;
    fn get_nodes(&self) -> &[Node<T>];
    fn get_node(&self, index: usize) -> Option<&Node<T>>;
    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>>;
    fn get_nodes_mut(&mut self) -> &mut [Node<T>];

    fn attach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().insert(outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().insert(incoming);
    }

    fn detach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().remove(&outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().remove(&incoming);
    }

    fn reindex(&self, index: usize) -> C;

    fn set_cycles(&mut self) -> C;
}
