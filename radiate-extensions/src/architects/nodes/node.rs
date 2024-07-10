use std::collections::HashSet;
use uuid::Uuid;
use crate::architects::node_types::NodeType;

pub trait Node<T> {
    fn id(&self) -> &Uuid;
    fn index(&self) -> &i32;
    fn node_type(&self) -> &NodeType;
    fn value(&self) -> &T;
    fn incoming(&mut self) -> &mut HashSet<i32>;
    fn outgoing(&mut self) -> &mut HashSet<i32>;
}
