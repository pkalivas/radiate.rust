use std::collections::HashSet;
use uuid::Uuid;
use crate::architects::node_types::NodeType;

pub trait Node<T> {
    fn new_node(index: usize, node_type: NodeType, value: T) -> Self;
    fn id(&self) -> &Uuid;
    fn index(&self) -> &usize;
    fn node_type(&self) -> &NodeType;
    fn value(&self) -> &T;
    fn reindex(&mut self, index: usize) -> Self;
    fn incoming_mut(&mut self) -> &mut HashSet<usize>;
    fn outgoing_mut(&mut self) -> &mut HashSet<usize>;
}
