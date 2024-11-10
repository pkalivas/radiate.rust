
use crate::architects::nodes::node::Node;

pub trait NodeCollection<C, T>
where
    C: NodeCollection<C, T> + Default,
    T: Clone + PartialEq + Default
{
    fn from_nodes(nodes: Vec<Node<T>>) -> Self;
    fn get_nodes(&self) -> &[Node<T>];
    fn get_node(&self, index: usize) -> Option<&Node<T>>;
    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>>;
    fn get_nodes_mut(&mut self) -> &mut [Node<T>];

    fn len(&self) -> usize {
        self.get_nodes().len()
    }

    fn attach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().insert(outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().insert(incoming);
    }

    fn detach(&mut self, incoming: usize, outgoing: usize) {
        self.get_nodes_mut()[incoming].outgoing_mut().remove(&outgoing);
        self.get_nodes_mut()[outgoing].incoming_mut().remove(&incoming);
    }

    fn reindex(&self, index: usize) -> C {
        let mut new_nodes = self.get_nodes()
            .iter()
            .enumerate()
            .map(|(i, node)| Node::new(index + i, node.node_type.clone(), node.value.clone()))
            .collect::<Vec<Node<T>>>();

        let old_nodes = self.get_nodes()
            .iter()
            .enumerate()
            .map(|(i, node)| (node.index, i))
            .collect::<std::collections::BTreeMap<usize, usize>>();

        for i in 0..new_nodes.len() {
            let old_node = self.get_node(i).unwrap();
            let new_node = &mut new_nodes[i];

            for incoming in old_node.incoming.iter() {
                if let Some(old_index) = old_nodes.get(incoming) {
                    let old_incoming = self.get_node(*old_index).unwrap();
                    new_node.incoming_mut().insert(old_incoming.index);
                }
            }

            for outgoing in old_node.outgoing.iter() {
                if let Some(old_index) = old_nodes.get(outgoing) {
                    let old_outgoing = self.get_node(*old_index).unwrap();
                    new_node.outgoing_mut().insert(old_outgoing.index);
                }
            }
        }

        C::from_nodes(new_nodes)
    }

    fn set_cycles(&mut self) -> C {
        unimplemented!()
    }
}
