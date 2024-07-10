// use std::collections::HashSet;
//
// use crate::architects::node_types::NodeType;
//
// use super::node::Node;
//
//
// pub struct GraphNode<T>
// where
//     T: Clone + PartialEq
// {
//     pub id: i32,
//     pub node_type: NodeType,
//     pub value: T,
//     pub incoming: HashSet<i32>,
//     pub outgoing: HashSet<i32>
// }
//
// impl<T> GraphNode<T>
// where
//     T: Clone + PartialEq
// {
//     pub fn new(id: i32, node_type: NodeType, value: T) -> GraphNode<T> {
//         GraphNode {
//             id,
//             node_type,
//             value,
//             incoming: HashSet::new(),
//             outgoing: HashSet::new()
//         }
//     }
// }
//
// impl<T> Node<T> for GraphNode<T>
// where
//     T: Clone + PartialEq
// {
//     fn index(&self) -> &i32 {
//         &self.id
//     }
//
//     fn node_type(&self) -> &NodeType {
//         &self.node_type
//     }
//
//     fn value(&self) -> &T {
//         &self.value
//     }
//
//     fn incoming(&mut self) -> &mut HashSet<i32> {
//         &mut self.incoming
//     }
//
//     fn outgoing(&mut self) -> &mut HashSet<i32> {
//         &mut self.outgoing
//     }
// }