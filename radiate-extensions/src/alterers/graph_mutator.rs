
use radiate_rust::engines::{alterers::mutators::mutate::Mutate, genome::{chromosome::Chromosome, genes::gene::Valid, genotype::Genotype}};
use rand::{random, seq::SliceRandom};

use crate::{architects::{node_collections::{self, graphs::graph::Graph, node::Node, node_collection::NodeCollection, node_factory::NodeFactory}, schema::node_types::NodeType}, operations::op::Ops};


pub struct NodeMutate {
    pub rate: f32,
    pub node_type: NodeType
}

pub struct GraphMutator<T> 
where
    T: Clone + PartialEq + Default
{
    pub factory: NodeFactory<T>,
    pub mutations: Vec<NodeMutate>,
}

impl<T> GraphMutator<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(factory: NodeFactory<T>) -> Self {
        GraphMutator {
            factory,
            mutations: vec![]
        }
    }

    pub fn add_mutation(mut self, node_type: NodeType, rate: f32) -> Self {
        self.mutations.push(NodeMutate { rate, node_type });
        self
    }

    pub fn mutate(&self, collection: Graph<T>, node_type: &NodeType) -> Option<Graph<T>> {
        let nodes = collection.get_nodes();
        let source_node = node_collections::random_source_node(nodes);
        let target_node = node_collections::random_target_node(nodes);

        if source_node.node_type == NodeType::Weight && node_type != &NodeType::Weight {
            let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
            let outgoing_node = collection.get(*source_node.outgoing.iter().next().unwrap()).unwrap();

            let new_source_edge = self.factory.new_node(collection.len(), source_node.node_type);
            let new_node = self.factory.new_node(collection.len() + 1, *node_type);
            let new_target_edge = self.factory.new_node(collection.len() + 2, source_node.node_type);

            if node_collections::is_locked(outgoing_node) {
                let mut temp = collection.insert(vec![new_source_edge.clone(), new_node.clone()]);

                temp.attach(source_node.index, new_node.index);
                temp.attach(new_node.index, new_source_edge.index);
                temp.attach(new_source_edge.index, outgoing_node.index);
                temp.detach(source_node.index, outgoing_node.index);

                return self.repair_insert(&mut temp, &new_node, incoming_node, outgoing_node);
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

                return self.repair_insert(&mut temp, &new_node, incoming_node, outgoing_node);
            }
        } else if !node_collections::can_connect(collection.get_nodes(), source_node.index, target_node.index) {
            return None;
        } 

        let new_node = self.factory.new_node(collection.len(), *node_type);

        let mut temp = collection.insert(vec![new_node.clone()]);

        temp.attach(source_node.index, new_node.index);
        temp.attach(new_node.index, target_node.index);
        temp.detach(source_node.index, target_node.index);
        temp.set_cycles(vec![source_node.index, target_node.index]);

        return self.repair_insert(&mut temp, &new_node, source_node, target_node);
    }

    fn repair_insert(&self,
        collection: &mut Graph<T>, 
        new_node: &Node<T>,
        source_node: &Node<T>,
        target_node: &Node<T>
    ) -> Option<Graph<T>>
    {
        for _ in 0..new_node.arity().unwrap() - 1 {
            let other_source_node = node_collections::random_source_node(collection.get_nodes());

            if node_collections::can_connect(collection.get_nodes(), other_source_node.index, new_node.index) {
                collection.attach(other_source_node.index, new_node.index);
            }
        }

        if !collection.is_valid() {
            return None;
        }

        return Some(collection.set_cycles(vec![source_node.index, target_node.index]).clone());
    }
}

impl<T> Mutate<Node<T>, Ops<T>> for GraphMutator<T>
where
    T: Clone + PartialEq + Default
{
    fn mutate_rate(&self) -> f32 {
        0.0
    }

    fn mutate_genotype(&self, genotype: &mut Genotype<Node<T>, Ops<T>>, _: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let mutation = self.mutations.choose(&mut rng).unwrap();

        if random::<f32>() < mutation.rate {
            let graph = Graph::from_nodes(genotype.iter()
                .next()
                .unwrap()
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<Node<T>>>());

            if let Some(mutated_graph) = self.mutate(graph, &mutation.node_type) {
                if !mutated_graph.is_valid() {
                    return 0;
                }

                genotype.chromosomes = vec![Chromosome::from_genes(mutated_graph.into_iter().collect::<Vec<Node<T>>>())];
                return 1;
            }
        }

        0
    }
}