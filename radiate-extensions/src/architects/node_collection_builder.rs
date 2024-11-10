use std::collections::BTreeMap;

use crate::architects::node_collections::node_collection::NodeCollection;
use crate::architects::schema::node_types::NodeType;
use crate::architects::nodes::node::Node;
use crate::architects::node_factory::NodeFactory;

use uuid::Uuid;


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
    pub nodes: BTreeMap<&'a Uuid, &'a N>,
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
            nodes: BTreeMap::new(),
            relationships: Vec::new(),
            _phantom_c: std::marker::PhantomData,
            _phantom_t: std::marker::PhantomData
        }
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

    pub fn build(self) -> C {
        let mut new_nodes = Vec::new();
        let mut node_id_index_map = BTreeMap::new();
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
        for node in one.get_nodes() {
            if !self.nodes.contains_key(node.id()) {
                self.nodes.insert(node.id(), node);
            }
        }

        for node in two.get_nodes() {
            if !self.nodes.contains_key(node.id()) {
                self.nodes.insert(node.id(), node);
            }
        }
        
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