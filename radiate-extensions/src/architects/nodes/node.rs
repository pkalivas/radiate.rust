use std::collections::HashSet;
use uuid::Uuid;
use crate::architects::node_types::NodeType;

pub trait Node<N, T> 
where 
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    fn new_node(index: usize, node_type: NodeType, value: T) -> N;
    fn id(&self) -> &Uuid;
    fn index(&self) -> &usize;
    fn node_type(&self) -> &NodeType;
    fn value(&self) -> &T;
    fn is_recurrent(&self) -> bool;
    fn incoming_mut(&mut self) -> &mut HashSet<usize>;
    fn outgoing_mut(&mut self) -> &mut HashSet<usize>;
    fn incoming(&self) -> &HashSet<usize>;
    fn outgoing(&self) -> &HashSet<usize>;
}
