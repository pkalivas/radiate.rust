use std::collections::HashMap;

use uuid::Uuid;

use super::{node_collections::node_collection::NodeCollection, node_types::NodeType, nodes::node::Node};

pub enum ConnectTypes {
    OneToOne,
    OneToMany,
    ManyToOne,
    AllToAll,
    AllToAllSelf,
    ParentToChild,
}



pub struct NodeRelationship {
    pub source_id: Uuid,
    pub target_id: Uuid,
}

pub struct NodeCollectionBuilder<C, N, T> 
where
    C: NodeCollection<C, N, T>,
    N: Node<N, T>,
    T: Clone + PartialEq
{
    pub nodes: HashMap<Uuid, N>,
    pub relationships: Vec<NodeRelationship>,
    _phantom_c: std::marker::PhantomData<C>,
    _phantom_t: std::marker::PhantomData<T>
}


impl<C, N, T> NodeCollectionBuilder<C, N, T> 
where
    C: NodeCollection<C, N, T>,
    N: Node<N, T>,
    T: Clone + PartialEq
{
    pub fn new() -> NodeCollectionBuilder<C, N, T> {
        NodeCollectionBuilder {
            nodes: HashMap::new(),
            relationships: Vec::new(),
            _phantom_c: std::marker::PhantomData,
            _phantom_t: std::marker::PhantomData
        }
    }
}





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