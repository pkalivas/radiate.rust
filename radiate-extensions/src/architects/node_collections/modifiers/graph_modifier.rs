
use crate::architects::{node_collections::{graphs::graph::Graph, modifiers::modifier, node_collection::NodeCollection, node_factory::NodeFactory}, schema::node_types::NodeType};

use super::modifier::Modifier;


pub struct Graphmodifier<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub factory: &'a NodeFactory<T>,
    pub node_type: NodeType
}

impl<'a, T> Graphmodifier<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(factory: &'a NodeFactory<T>, node_type: NodeType) -> Self {
        Graphmodifier { factory, node_type }
    }

    // pub fn repair_insert(&self, original: Graph<T>, collection: Graph<T>, source_node: usize, target_node: usize) -> Graph<T> {
    //     let source_node = collection.get(source_node).unwrap();
    //     let target_node = collection.get(target_node).unwrap();

    //    unimplemented!()
    // }
}

impl<'a, T> Modifier<Graph<T>, T> for Graphmodifier<'a, T>
where
    T: Clone + PartialEq + Default
{
    fn modify(&self, collection: &mut Graph<T>) -> Graph<T> {
        let source_node = modifier::random_source_node(collection);
        let target_node = modifier::random_target_node(collection);

        if source_node.node_type == NodeType::Weight && self.node_type != NodeType::Weight {
            // let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
            let outgoing_node = collection.get(*source_node.outgoing.iter().next().unwrap()).unwrap();

            let new_source_edge = self.factory.new_node(collection.len(), source_node.node_type);
            let new_node = self.factory.new_node(collection.len() + 1, self.node_type);
            let new_target_edge = self.factory.new_node(collection.len() + 2, source_node.node_type);

            if modifier::is_locked(outgoing_node) {
                let mut temp = collection.insert(vec![new_source_edge.clone(), new_node.clone()]);

                temp.attach(source_node.index, new_node.index);
                temp.attach(new_node.index, new_source_edge.index);
                temp.attach(new_source_edge.index, outgoing_node.index);
                temp.detach(source_node.index, outgoing_node.index);

                return temp;
            } else {
                let mut temp = collection.insert(vec![new_source_edge.clone(), new_node.clone(), new_target_edge.clone()]);

                temp.attach(source_node.index, new_source_edge.index);
                temp.attach(source_node.index, new_node.index);
                temp.attach(new_node.index, new_target_edge.index);
                temp.attach(new_target_edge.index, outgoing_node.index);

                return temp;
                // temp = temp.set_cycles(vec![incoming_node.index, outgoing_node.index]);
            }
        } else if !modifier::can_connect(collection, source_node.index, target_node.index) {
            return collection.clone();
        } 

        let new_node = self.factory.new_node(collection.len(), self.node_type);

        let mut temp = collection.insert(vec![new_node.clone()]);

        temp.attach(source_node.index, new_node.index);
        temp.attach(new_node.index, target_node.index);
        temp.detach(source_node.index, target_node.index);

        return temp;
    }
}
