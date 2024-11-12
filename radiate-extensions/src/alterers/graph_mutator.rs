use radiate_rust::engines::{alterers::mutators::mutate::Mutate, genome::{chromosome::Chromosome, genes::gene::Valid, genotype::Genotype}};
use rand::random;

use crate::{architects::{node_collections::{graphs::graph::Graph, modifiers::modifier, node::Node, node_collection::NodeCollection, node_factory::NodeFactory}, schema::node_types::NodeType}, operations::op::Ops};


pub struct GraphMutator<T> 
where
    T: Clone + PartialEq + Default
{
    pub rate: f32,
    pub factory: NodeFactory<T>,
    pub node_type: NodeType
}

impl<T> GraphMutator<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(rate: f32, factory: NodeFactory<T>, node_type: NodeType) -> GraphMutator<T> {
        GraphMutator {
            rate,
            factory,
            node_type
        }
    }

    pub fn mutate(&self, collection: &Graph<T>) -> Graph<T> {
        let source_node = modifier::random_source_node(collection.get_nodes());
        let target_node = modifier::random_target_node(collection.get_nodes());

        if source_node.node_type == NodeType::Weight && self.node_type != NodeType::Weight {
            let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
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

                return self.repair_insert(collection, &mut temp, &new_node, incoming_node, outgoing_node);
            } else {
                let mut temp = collection.insert(vec![
                    new_source_edge.clone(), 
                    new_node.clone(),
                    new_target_edge.clone()
                ]);

                temp.attach(source_node.index, new_source_edge.index);
                temp.attach(source_node.index, new_node.index);
                temp.attach(new_node.index, new_target_edge.index);
                temp.attach(new_target_edge.index, outgoing_node.index);

                return self.repair_insert(collection, &mut temp, &new_node, incoming_node, outgoing_node);
            }
        } else if !modifier::can_connect(collection.get_nodes(), source_node.index, target_node.index) {
            return collection.clone();
        } 

        let new_node = self.factory.new_node(collection.len(), self.node_type);

        let mut temp = collection.insert(vec![new_node.clone()]);

        temp.attach(source_node.index, new_node.index);
        temp.attach(new_node.index, target_node.index);
        temp.detach(source_node.index, target_node.index);
        temp.set_cycles(vec![source_node.index, target_node.index]);

        return self.repair_insert(collection, &mut temp, &new_node, source_node, target_node);
    }

    fn repair_insert(&self, 
        original: &Graph<T>,
        collection: &mut Graph<T>, 
        new_node: &Node<T>,
        source_node: &Node<T>,
        target_node: &Node<T>
    ) -> Graph<T>
    {
        for _ in 0..new_node.arity().unwrap() - 1 {
            let other_source_node = modifier::random_source_node(collection.get_nodes());

            if modifier::can_connect(collection.get_nodes(), other_source_node.index, new_node.index) {
                collection.attach(other_source_node.index, new_node.index);
            }
        }

        if !collection.is_valid() {
            return original.clone();
        }

        return collection.set_cycles(vec![source_node.index, target_node.index]).clone()
    }
}

impl<T> Mutate<Node<T>, Ops<T>> for GraphMutator<T>
where
    T: Clone + PartialEq + Default
{
    fn mutate_rate(&self) -> f32 {
        self.rate
    }

    fn mutate_genotype(&self, genotype: &mut Genotype<Node<T>, Ops<T>>, _: i32, _: f32) -> i32 {
        if random::<f32>() < self.rate {
            let mut graph = Graph::from_nodes(genotype.iter()
                .next()
                .unwrap()
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<Node<T>>>());

            graph = self.mutate(&graph);

            if !graph.is_valid() {
                return 0;
            }

            genotype.chromosomes = vec![Chromosome::from_genes(graph.get_nodes().iter().map(|node| node.clone()).collect::<Vec<Node<T>>>())];

            return 1;
        }

        0
    }
}