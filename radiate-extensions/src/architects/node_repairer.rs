use super::node_collections::node_collection::NodeCollection;


pub fn arity_node_repairer<C, T>(collection: &mut C) -> C
where
    C: NodeCollection<C, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    for node in collection.get_nodes_mut() {
        let arity = node.incoming().len();
        (*node).arity = Some(arity as u8);
    }

    collection.clone()
}