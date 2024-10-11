use std::collections::HashSet;
use uuid::Uuid;
use crate::architects::node_types::NodeType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Id(usize);

pub trait Node<N, T> 
where 
    N: Node<N, T>,
    T: Clone + PartialEq
{
    fn new_node(index: usize, node_type: NodeType, value: T) -> N;
    fn id(&self) -> &Uuid;
    fn index(&self) -> &usize;
    fn node_type(&self) -> &NodeType;
    fn value(&self) -> &T;
    fn incoming_mut(&mut self) -> &mut HashSet<usize>;
    fn outgoing_mut(&mut self) -> &mut HashSet<usize>;
}
