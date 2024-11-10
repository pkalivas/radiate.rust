use std::collections::HashMap;

use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::node_types::NodeType;
use crate::architects::nodes::node::Node;

use uuid::Uuid;

use super::node_factory::NodeFactory;


pub enum ConnectTypes {
    OneToOne,
    OneToMany,
    ManyToOne,
    AllToAll,
    AllToAllSelf,
    ParentToChild,
}

pub struct NodeRelationship<'a> {
    pub source_id: &'a Uuid,
    pub target_id: &'a Uuid,
}

pub struct NodeCollectionBuilder<'a, C, N, T> 
where
    C: NodeCollection<C, N, T> + Default,
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub factory: &'a NodeFactory<T>,
    pub nodes: HashMap<&'a Uuid, &'a N>,
    pub relationships: Vec<NodeRelationship<'a>>,
    _phantom_c: std::marker::PhantomData<C>,
    _phantom_t: std::marker::PhantomData<T>
}


impl<'a, C, N, T> NodeCollectionBuilder<'a, C, N, T> 
where
    C: NodeCollection<C, N, T> + Default,
    N: Node<N, T> + Clone + Default,
    T: Clone + PartialEq + Default
{
    pub fn new(factory: &'a NodeFactory<T>) -> NodeCollectionBuilder<'a, C, N, T> {
        NodeCollectionBuilder {
            factory,
            nodes: HashMap::new(),
            relationships: Vec::new(),
            _phantom_c: std::marker::PhantomData,
            _phantom_t: std::marker::PhantomData
        }
    }

    pub fn input(&self, siez: usize) -> C {
        self.new_collection(NodeType::Input, siez)
    }

    pub fn output(&self, siez: usize) -> C {
        self.new_collection(NodeType::Output, siez)
    }

    pub fn gate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Gate, siez)
    }

    pub fn aggregate(&self, siez: usize) -> C {
        self.new_collection(NodeType::Aggregate, siez)
    }

    pub fn weight(&self, siez: usize) -> C {
        self.new_collection(NodeType::Weight, siez)
    }

    fn new_collection(&self, node_type: NodeType, size: usize) -> C {
        let nodes = self.new_nodes(node_type, size);

        C::from_nodes(nodes)
    }

    fn new_nodes(&self, node_type: NodeType, size: usize) -> Vec<N> {
        let mut nodes = Vec::new();

        for i in 0..size {
            nodes.push(self.factory.new_node(i, node_type));
        }

        nodes
    }

    pub fn one_to_one(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::OneToOne, one, two);
        self
    }

    pub fn one_to_many(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::OneToMany, one, two);
        self
    }

    pub fn many_to_one(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::ManyToOne, one, two);
        self
    }

    pub fn all_to_all(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::AllToAll, one, two);
        self
    }

    pub fn self_connt(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::AllToAllSelf, one, two);
        self
    }

    pub fn parent_to_child(mut self, one: &'a C, two: &'a C) -> Self {
        self.attach(ConnectTypes::ParentToChild, one, two);
        self
    }

    pub fn build(mut self) -> C {
        let mut new_nodes = Vec::new();
        let mut node_id_index_map = HashMap::new();
        let mut idx = 0;

        for (id, node) in self.nodes.iter() {
            let new_node = N::new_node(idx, *node.node_type(), node.value().clone());

            new_nodes.push(new_node);
            node_id_index_map.insert(id, idx);

            idx += 1;
        }

        let mut new_collection = C::from_nodes(new_nodes);

        for rel in self.relationships {
            let source_idx = node_id_index_map.get(&rel.source_id).unwrap();
            let target_idx = node_id_index_map.get(&rel.target_id).unwrap();

            new_collection.attach(*source_idx, *target_idx);
        }

        new_collection
    }

    fn attach(&mut self, connection: ConnectTypes, one: &'a C, two: &'a C) {
        match connection {
            ConnectTypes::OneToOne => self.one_to_one_connect(one, two),
            ConnectTypes::OneToMany => self.one_to_many_connect(one, two),
            ConnectTypes::ManyToOne => self.many_to_one_connect(one, two),
            ConnectTypes::AllToAll => self.all_to_all_connect(one, two),
            ConnectTypes::AllToAllSelf => self.self_connect(one, two),
            ConnectTypes::ParentToChild => self.parent_to_child_connect(one, two),
        }
    }

    fn one_to_one_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        if one_outputs.len() != two_inputs.len() {
            panic!(
                "OneToOne - oneGroup outputs must be the same length as twoGroup inputs."
            );
        }

        for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
            self.relationships.push(NodeRelationship {
                source_id: one.id(),
                target_id: two.id(),
            });
        }
    }

    fn one_to_many_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        if two_inputs.len() % one_outputs.len() != 0 {
            panic!("OneToMany - TwoGroup inputs must be a multiple of OneGroup outputs.");
        }

        for targets in two_inputs.chunks(one_outputs.len()) {
            for (source, target) in one_outputs.iter().zip(targets.iter()) {
                self.relationships.push(NodeRelationship {
                    source_id: source.id(),
                    target_id: target.id(),
                });
            }
        }
    }

    fn many_to_one_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        if one_outputs.len() % two_inputs.len() != 0 {
            panic!("ManyToOne - OneGroup outputs must be a multiple of TwoGroup inputs.");
        }

        for sources in one_outputs.chunks(two_inputs.len()) {
            for (source, target) in sources.iter().zip(two_inputs.iter()) {
                self.relationships.push(NodeRelationship {
                    source_id: source.id(),
                    target_id: target.id()
                });
            }
        }
    }

    fn all_to_all_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        for source in one_outputs {
            for target in two_inputs.iter() {
                self.relationships.push(NodeRelationship {
                    source_id: source.id(),
                    target_id: target.id()
                });
            }
        }
    }

    fn self_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        if one_outputs.len() != two_inputs.len() {
            panic!("Self - oneGroup outputs must be the same length as twoGroup inputs.");
        }

        for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
            self.relationships.push(NodeRelationship {
                source_id: one.id(),
                target_id: two.id()
            });
            self.relationships.push(NodeRelationship {
                source_id: two.id(),
                target_id: one.id(),
            });
        }
    }

    fn parent_to_child_connect(&mut self, one: &'a C, two: &'a C) {
        let one_outputs = self.get_outputs(one);
        let two_inputs = self.get_inputs(two);

        if one_outputs.len() != 1 {
            panic!("ParentToChild - oneGroup outputs must be a single node.");
        }

        let parent_node = one_outputs[0];
        for child_node in two_inputs {
            self.relationships.push(NodeRelationship {
                source_id: parent_node.id(),
                target_id: child_node.id(),
            });
        }
    }

    fn get_outputs(&self, collection: &'a C) -> Vec<&'a N> {
        let outputs = collection
            .get_nodes()
            .iter()
            .enumerate()
            .skip_while(|(_, node)| node.outgoing().len() > 0)
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>();

        if outputs.len() > 0 {
            return outputs;
        }

        let recurrent_outputs = collection
            .get_nodes()
            .iter()
            .enumerate()
            .filter(|(_, node)| node.outgoing().len() == 1 
                && node.is_recurrent() 
                && (node.node_type() == &NodeType::Gate || node.node_type() == &NodeType::Aggregate))
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>();

        if recurrent_outputs.len() > 0 {
            return recurrent_outputs;
        }

        collection
            .get_nodes()
            .iter()
            .enumerate()
            .filter(|(_, node)| node.incoming().len() == 0)
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>()
    }


    fn get_inputs(&self, collection: &'a C) -> Vec<&'a N> {
        let inputs = collection
            .get_nodes()
            .iter()
            .enumerate()
            .skip_while(|(_, node)| node.incoming().len() > 0)
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>();

        if inputs.len() > 0 {
            return inputs;
        }

        let recurrent_inputs = collection
            .get_nodes()
            .iter()
            .enumerate()
            .filter(|(_, node)| node.outgoing().len() == 1 
                && node.is_recurrent() 
                && node.node_type() == &NodeType::Gate)
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>();

        if recurrent_inputs.len() > 0 {
            return recurrent_inputs;
        }

        collection
            .get_nodes()
            .iter()
            .enumerate()
            .filter(|(_, node)| node.outgoing().len() == 0)
            .map(|(idx, _)| collection.get_node(idx).unwrap())
            .collect::<Vec<&N>>()
    }
}





//
// struct NodeCollectionBuilder<T> {
//     factory: Box<dyn Fn(Id) -> T>,
//     nodes: HashMap<Id, Node<T>>,
//     relationships: Vec<(Id, Id)>,
// }
//
// impl<T> NodeCollectionBuilder<T> {
//     fn new(factory: Box<dyn Fn(Id) -> T>) -> Self {
//         Self {
//             factory,
//             nodes: HashMap::new(),
//             relationships: Vec::new(),
//         }
//     }
//
//     fn one_to_one(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::OneToOne, one, two);
//         self
//     }
//
//     fn one_to_many(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::OneToMany, one, two);
//         self
//     }
//
//     fn many_to_one(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::ManyToOne, one, two);
//         self
//     }
//
//     fn all_to_all(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::AllToAll, one, two);
//         self
//     }
//
//     fn self_connect(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::Self, one, two);
//         self
//     }
//
//     fn parent_to_child(mut self, one: &C, two: &C) -> Self {
//         self.attach(ConnectType::ParentToChild, one, two);
//         self
//     }
//
//     fn build(self) -> NodeCollection<T> {
//         let mut new_collection = NodeCollection::new();
//
//         for (id, node) in self.nodes {
//             new_collection.add_node(node);
//         }
//
//         for (source, target) in self.relationships {
//             if let Some(source_node) = new_collection.nodes.get_mut(&source) {
//                 source_node.outgoing.push(target.0);
//             }
//             if let Some(target_node) = new_collection.nodes.get_mut(&target) {
//                 target_node.incoming.push(source.0);
//             }
//         }
//
//         new_collection.reindex();
//         new_collection.set_cycles();
//
//         new_collection
//     }
//
//     fn attach(&mut self, connection: ConnectType, one: &C, two: &C) {
//         match connection {
//             ConnectType::OneToOne => self.one_to_one_connect(one, two),
//             ConnectType::OneToMany => self.one_to_many_connect(one, two),
//             ConnectType::ManyToOne => self.many_to_one_connect(one, two),
//             ConnectType::AllToAll => self.all_to_all_connect(one, two),
//             ConnectType::Self => self.self_connect(one, two),
//             ConnectType::ParentToChild => self.parent_to_child_connect(one, two),
//         }
//     }
//
//     fn one_to_one_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         if one_outputs.len() != two_inputs.len() {
//             panic!(
//                 "OneToOne - oneGroup outputs must be the same length as twoGroup inputs."
//             );
//         }
//
//         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
//             self.relationships.push((one, two));
//         }
//     }
//
//     fn one_to_many_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         if two_inputs.len() % one_outputs.len() != 0 {
//             panic!("OneToMany - TwoGroup inputs must be a multiple of OneGroup outputs.");
//         }
//
//         for targets in two_inputs.chunks(one_outputs.len()) {
//             for (source, target) in one_outputs.iter().zip(targets.iter()) {
//                 self.relationships.push((*source, *target));
//             }
//         }
//     }
//
//     fn many_to_one_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         if one_outputs.len() % two_inputs.len() != 0 {
//             panic!("ManyToOne - OneGroup outputs must be a multiple of TwoGroup inputs.");
//         }
//
//         for sources in one_outputs.chunks(two_inputs.len()) {
//             for (source, target) in sources.iter().zip(two_inputs.iter()) {
//                 self.relationships.push((*source, *target));
//             }
//         }
//     }
//
//     fn all_to_all_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         for source in one_outputs {
//             for target in two_inputs.iter() {
//                 self.relationships.push((source, *target));
//             }
//         }
//     }
//
//     fn self_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         if one_outputs.len() != two_inputs.len() {
//             panic!("Self - oneGroup outputs must be the same length as twoGroup inputs.");
//         }
//
//         for (one, two) in one_outputs.into_iter().zip(two_inputs.into_iter()) {
//             self.relationships.push((one, two));
//             self.relationships.push((two, one));
//         }
//     }
//
//     fn parent_to_child_connect(&mut self, one: &C, two: &C) {
//         let one_outputs = self.get_outputs(one);
//         let two_inputs = self.get_inputs(two);
//
//         if one_outputs.len() != 1 {
//             panic!("ParentToChild - oneGroup outputs must be a single node.");
//         }
//
//         let parent_node = one_outputs[0];
//         for child_node in two_inputs {
//             self.relationships.push((parent_node, child_node));
//         }
//     }
//
//     fn get_outputs(&self, collection: &C) -> Vec<Id> {
//         collection
//             .nodes
//             .iter()
//             .filter_map(|(_, node)| {
//                 if node.outgoing.is_empty() {
//                     Some(node.id)
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }
//
//     fn get_inputs(&self, collection: &C) -> Vec<Id> {
//         collection
//             .nodes
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
//

    // fn parent_to_child_connect(&mut self, group_one: C, group_two: C) {
    //     let one_outputs = group_one.outputs();
    //     let two_inputs = group_two.inputs();

    //     assert_eq!(one_outputs.len(), 1, "ParentToChild - oneGroup outputs must be a single node. {} != 1", one_outputs.len());

    //     let parent_node = one_outputs[0].clone();
    //     for child_node in two_inputs.iter() {
    //         self.relationships.insert(NodeRelationship {
    //             source_node_id: parent_node.id().clone(),
    //             target_node_id: child_node.id().clone(),
    //         });
    //     }
    // }

    // fn get_outputs(collection: &'static mut C) -> Vec<&'static mut N> {
    //     let outputs = collection
    //         .get_nodes()
    //         .iter()
    //         .enumerate()
    //         .skip_while(|(idx, node)| node.outgoing_mut().len() > 0)
    //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
    //         .collect::<Vec<&mut N>>();

    //     if outputs.len() > 0 {
    //         return outputs;
    //     }

    //     let recurrent_outputs = collection
    //         .get_nodes()
    //         .iter()
    //         .enumerate()
    //         .filter(|(idx, node)| node.outgoing_mut().len() == 1 && node.node_type() == &NodeType::Gate)//&& node.is_recurrent())
    //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
    //         .collect::<Vec<&mut N>>();

    //     if recurrent_outputs.len() > 0 {
    //         return recurrent_outputs;
    //     }

    //     collection
    //         .get_nodes()
    //         .iter()
    //         .enumerate()
    //         .filter(|(idx, node)| node.incoming_mut().len() == 0)
    //         .map(|(idx, node)| collection.get_node_mut(idx).unwrap())
    //         .collect::<Vec<&mut N>>()
    // }