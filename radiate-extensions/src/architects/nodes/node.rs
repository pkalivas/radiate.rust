use std::collections::HashSet;

use crate::architects::node_types::NodeType;

pub trait Node<T> {
    fn id(&self) -> &i32;
    fn node_type(&self) -> &NodeType;
    fn value(&self) -> &T;
    fn incoming(&self) -> &HashSet<i32>;
    fn outgoing(&self) -> &HashSet<i32>;
}
