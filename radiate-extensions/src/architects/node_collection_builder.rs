// use std::collections::HashMap;

// use uuid::Uuid;

// use super::{node_collections::node_collection::NodeCollection, node_types::NodeType, nodes::node::Node};

// pub enum ConnectTypes {
//     OneToOne,
//     OneToMany,
//     ManyToOne,
//     AllToAll,
//     AllToAllSelf,
//     ParentToChild,
// }

// pub struct NodeRelationship {
//     pub source_id: Uuid,
//     pub target_id: Uuid,
// }

// pub struct NodeCollectionBuilder<C, N, T> 
// where
//     C: NodeCollection<C, N, T>,
//     N: Node<N, T>,
//     T: Clone + PartialEq
// {
//     pub factory: Box<dyn Fn(usize, NodeType) -> N>,
//     pub nodes: HashMap<Uuid, N>,
//     pub relationships: Vec<NodeRelationship>,
//     _phantom_c: std::marker::PhantomData<C>,
//     _phantom_t: std::marker::PhantomData<T>
// }


// impl<C, N, T> NodeCollectionBuilder<C, N, T> 
// where
//     C: NodeCollection<C, N, T>,
//     N: Node<N, T>,
//     T: Clone + PartialEq
// {
//     pub fn new(factory: Box<dyn Fn(usize, NodeType) -> N>) -> NodeCollectionBuilder<C, N, T> {
//         NodeCollectionBuilder {
//             factory,
//             nodes: HashMap::new(),
//             relationships: Vec::new(),
//             _phantom_c: std::marker::PhantomData,
//             _phantom_t: std::marker::PhantomData
//         }
//     }

//     fn one_to_one(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::OneToOne, one, two);
//         self
//     }

//     fn one_to_many(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::OneToMany, one, two);
//         self
//     }

//     fn many_to_one(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::ManyToOne, one, two);
//         self
//     }

//     fn all_to_all(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::AllToAll, one, two);
//         self
//     }

//     fn self_connt(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::AllToAllSelf, one, two);
//         self
//     }

//     fn parent_to_child(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectTypes::ParentToChild, one, two);
//         self
//     }

//     fn build(self) -> C {
//         let mut new_collection = NodeCollection::new();

//         for (id, node) in self.nodes {
//             new_collection.add_node(node);
//         }

//         for (source, target) in self.relationships {
//             if let Some(source_node) = new_collection.nodes.get_mut(&source) {
//                 source_node.outgoing.push(target.0);
//             }
//             if let Some(target_node) = new_collection.nodes.get_mut(&target) {
//                 target_node.incoming.push(source.0);
//             }
//         }

//         new_collection.reindex();
//         new_collection.set_cycles();

//         new_collection
//     }

//     fn attach(&mut self, connection: ConnectTypes, one: &C, two: &C) {
//         match connection {
//             ConnectTypes::OneToOne => self.one_to_one_connect(one, two),
//             ConnectTypes::OneToMany => self.one_to_many_connect(one, two),
//             ConnectTypes::ManyToOne => self.many_to_one_connect(one, two),
//             ConnectTypes::AllToAll => self.all_to_all_connect(one, two),
//             ConnectTypes::AllToAllSelf => self.self_connect(one, two),
//             ConnectTypes::ParentToChild => self.parent_to_child_connect(one, two),
//         }
//     }

//     fn one_to_one_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         if one_outputs.len() != two_inputs.len() {
//             panic!(
//                 "OneToOne - oneGroup outputs must be the same length as twoGroup inputs."
//             );
//         }

//         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
//             self.relationships.push(NodeRelationship {
//                 source_id: one,
//                 target_id: two,
//             });
//         }
//     }

//     fn one_to_many_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         if two_inputs.len() % one_outputs.len() != 0 {
//             panic!("OneToMany - TwoGroup inputs must be a multiple of OneGroup outputs.");
//         }

//         for targets in two_inputs.chunks(one_outputs.len()) {
//             for (source, target) in one_outputs.iter().zip(targets.iter()) {
//                 self.relationships.push(NodeRelationship {
//                     source_id: *source,
//                     target_id: *target,
//                 });
//             }
//         }
//     }

//     fn many_to_one_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         if one_outputs.len() % two_inputs.len() != 0 {
//             panic!("ManyToOne - OneGroup outputs must be a multiple of TwoGroup inputs.");
//         }

//         for sources in one_outputs.chunks(two_inputs.len()) {
//             for (source, target) in sources.iter().zip(two_inputs.iter()) {
//                 self.relationships.push(NodeRelationship {
//                     source_id: *source,
//                     target_id: *target,
//                 });
//             }
//         }
//     }

//     fn all_to_all_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         for source in one_outputs {
//             for target in two_inputs.iter() {
//                 self.relationships.push(NodeRelationship {
//                     source_id: source,
//                     target_id: *target,
//                 });
//             }
//         }
//     }

//     fn self_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         if one_outputs.len() != two_inputs.len() {
//             panic!("Self - oneGroup outputs must be the same length as twoGroup inputs.");
//         }

//         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
//             self.relationships.push(NodeRelationship {
//                 source_id: one,
//                 target_id: two,
//             });
//             self.relationships.push(NodeRelationship {
//                 source_id: two,
//                 target_id: one,
//             });
//         }
//     }

//     fn parent_to_child_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);

//         if one_outputs.len() != 1 {
//             panic!("ParentToChild - oneGroup outputs must be a single node.");
//         }

//         let parent_node = one_outputs[0];
//         for child_node in two_inputs {
//             self.relationships.push(NodeRelationship {
//                 source_id: parent_node,
//                 target_id: child_node,
//             });
//         }
//     }

//     fn get_outputs(&self, collection: &C) -> Vec<Uuid> {
//         collection
//             .get_nodes()
//             .iter()
//             .filter_map(|(_, node)| {
//                 if node.outgoing.is_empty() {
//                     Some(node.Id)
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }

//     fn get_inputs(&self, collection: &C) -> Vec<Uuid> {
//         collection
//             .get_nodes()
//             .iter()
//             .filter_map(|(_, node)| {
//                 if node.incoming.is_empty() {
//                     Some(node.id)
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }
// }

// //
// // struct NodeCollectionBuilder<T> {
// //     factory: Box<dyn Fn(Id) -> T>,
// //     nodes: HashMap<Id, Node<T>>,
// //     relationships: Vec<(Id, Id)>,
// // }
// //
// // impl<T> NodeCollectionBuilder<T> {
// //     fn new(factory: Box<dyn Fn(Id) -> T>) -> Self {
// //         Self {
// //             factory,
// //             nodes: HashMap::new(),
// //             relationships: Vec::new(),
// //         }
// //     }
// //
// //     fn one_to_one(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::OneToOne, one, two);
// //         self
// //     }
// //
// //     fn one_to_many(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::OneToMany, one, two);
// //         self
// //     }
// //
// //     fn many_to_one(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::ManyToOne, one, two);
// //         self
// //     }
// //
// //     fn all_to_all(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::AllToAll, one, two);
// //         self
// //     }
// //
// //     fn self_connect(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::Self, one, two);
// //         self
// //     }
// //
// //     fn parent_to_child(mut self, one: &C, two: &C) -> Self {
// //         self.attach(ConnectType::ParentToChild, one, two);
// //         self
// //     }
// //
// //     fn build(self) -> NodeCollection<T> {
// //         let mut new_collection = NodeCollection::new();
// //
// //         for (id, node) in self.nodes {
// //             new_collection.add_node(node);
// //         }
// //
// //         for (source, target) in self.relationships {
// //             if let Some(source_node) = new_collection.nodes.get_mut(&source) {
// //                 source_node.outgoing.push(target.0);
// //             }
// //             if let Some(target_node) = new_collection.nodes.get_mut(&target) {
// //                 target_node.incoming.push(source.0);
// //             }
// //         }
// //
// //         new_collection.reindex();
// //         new_collection.set_cycles();
// //
// //         new_collection
// //     }
// //
// //     fn attach(&mut self, connection: ConnectType, one: &C, two: &C) {
// //         match connection {
// //             ConnectType::OneToOne => self.one_to_one_connect(one, two),
// //             ConnectType::OneToMany => self.one_to_many_connect(one, two),
// //             ConnectType::ManyToOne => self.many_to_one_connect(one, two),
// //             ConnectType::AllToAll => self.all_to_all_connect(one, two),
// //             ConnectType::Self => self.self_connect(one, two),
// //             ConnectType::ParentToChild => self.parent_to_child_connect(one, two),
// //         }
// //     }
// //
// //     fn one_to_one_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         if one_outputs.len() != two_inputs.len() {
// //             panic!(
// //                 "OneToOne - oneGroup outputs must be the same length as twoGroup inputs."
// //             );
// //         }
// //
// //         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
// //             self.relationships.push((one, two));
// //         }
// //     }
// //
// //     fn one_to_many_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         if two_inputs.len() % one_outputs.len() != 0 {
// //             panic!("OneToMany - TwoGroup inputs must be a multiple of OneGroup outputs.");
// //         }
// //
// //         for targets in two_inputs.chunks(one_outputs.len()) {
// //             for (source, target) in one_outputs.iter().zip(targets.iter()) {
// //                 self.relationships.push((*source, *target));
// //             }
// //         }
// //     }
// //
// //     fn many_to_one_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         if one_outputs.len() % two_inputs.len() != 0 {
// //             panic!("ManyToOne - OneGroup outputs must be a multiple of TwoGroup inputs.");
// //         }
// //
// //         for sources in one_outputs.chunks(two_inputs.len()) {
// //             for (source, target) in sources.iter().zip(two_inputs.iter()) {
// //                 self.relationships.push((*source, *target));
// //             }
// //         }
// //     }
// //
// //     fn all_to_all_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         for source in one_outputs {
// //             for target in two_inputs.iter() {
// //                 self.relationships.push((source, *target));
// //             }
// //         }
// //     }
// //
// //     fn self_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         if one_outputs.len() != two_inputs.len() {
// //             panic!("Self - oneGroup outputs must be the same length as twoGroup inputs.");
// //         }
// //
// //         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
// //             self.relationships.push((one, two));
// //             self.relationships.push((two, one));
// //         }
// //     }
// //
// //     fn parent_to_child_connect(&mut self, one: &C, two: &C) {
// //         let one_outputs = self.get_outputs(one);
// //         let two_inputs = self.get_inputs(two);
// //
// //         if one_outputs.len() != 1 {
// //             panic!("ParentToChild - oneGroup outputs must be a single node.");
// //         }
// //
// //         let parent_node = one_outputs[0];
// //         for child_node in two_inputs {
// //             self.relationships.push((parent_node, child_node));
// //         }
// //     }
// //
// //     fn get_outputs(&self, collection: &C) -> Vec<Id> {
// //         collection
// //             .nodes
// //             .iter()
// //             .filter_map(|(_, node)| {
// //                 if node.outgoing.is_empty() {
// //                     Some(node.id)
// //                 } else {
// //                     None
// //                 }
// //             })
// //             .collect()
// //     }
// //
// //     fn get_inputs(&self, collection: &C) -> Vec<Id> {
// //         collection
// //             .nodes
// //             .iter()
// //             .filter_map(|(_, node)| {
// //                 if node.incoming.is_empty() {
// //                     Some(node.id)
// //                 } else {
// //                     None
// //                 }
// //             })
// //             .collect()
// //     }
// // }
// //

//     // fn parent_to_child_connect(&mut self, group_one: C, group_two: C) {
//     //     let one_outputs = group_one.outputs();
//     //     let two_inputs = group_two.inputs();

//     //     assert_eq!(one_outputs.len(), 1, "ParentToChild - oneGroup outputs must be a single node. {} != 1", one_outputs.len());

//     //     let parent_node = one_outputs[0].clone();
//     //     for child_node in two_inputs.iter() {
//     //         self.relationships.insert(NodeRelationship {
//     //             source_node_id: parent_node.id().clone(),
//     //             target_node_id: child_node.id().clone(),
//     //         });
//     //     }
//     // }

//     // fn get_outputs(collection: &'static mut C) -> Vec<&'static mut N> {
//     //     let outputs = collection
//     //         .get_nodes()
//     //         .iter()
//     //         .enumerate()
//     //         .skip_while(|(idx, node)| node.outgoing_mut().len() > 0)
//     //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
//     //         .collect::<Vec<&mut N>>();

//     //     if outputs.len() > 0 {
//     //         return outputs;
//     //     }

//     //     let recurrent_outputs = collection
//     //         .get_nodes()
//     //         .iter()
//     //         .enumerate()
//     //         .filter(|(idx, node)| node.outgoing_mut().len() == 1 && node.node_type() == &NodeType::Gate)//&& node.is_recurrent())
//     //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
//     //         .collect::<Vec<&mut N>>();

//     //     if recurrent_outputs.len() > 0 {
//     //         return recurrent_outputs;
//     //     }

//     //     collection
//     //         .get_nodes()
//     //         .iter()
//     //         .enumerate()
//     //         .filter(|(idx, node)| node.incoming_mut().len() == 0)
//     //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
//     //         .collect::<Vec<&mut N>>()
//     // }