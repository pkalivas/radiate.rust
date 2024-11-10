use crate::architects::{nodes::node::Node, schema::node_types::NodeType};


pub trait NodeFactory<T> {
    fn new_node(&self, index: usize, node_type: NodeType) -> Node<T> 
    where
        T: Clone + PartialEq + Default;
}
