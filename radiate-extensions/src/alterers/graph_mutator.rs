
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
        let mut temp = collection.clone();

        let nodes = collection.get_nodes();
        let source_node = node_collections::random_source_node(nodes);
        let target_node = node_collections::random_target_node(nodes);
        let source_node_index = source_node.index;
        let target_node_index = target_node.index;

        let new_source_edge_index = collection.len();
        let new_node_index = collection.len() + 1;
        let new_target_edge_index = collection.len() + 2;

        if source_node.node_type == NodeType::Weight && node_type != &NodeType::Weight {
            let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
            let outgoing_node = collection.get(*source_node.outgoing.iter().next().unwrap()).unwrap();

            let new_source_edge = self.factory.new_node(new_source_edge_index, source_node.node_type);
            let new_node = self.factory.new_node(new_node_index, *node_type);
            let new_target_edge = self.factory.new_node(new_target_edge_index, source_node.node_type);

            if node_collections::is_locked(outgoing_node) {
                temp.insert(vec![
                    new_source_edge,
                    new_node
                ]);

                temp.attach(source_node_index, new_node_index);
                temp.attach(new_node_index, new_source_edge_index);
                temp.attach(new_source_edge_index, outgoing_node.index);
                temp.detach(source_node_index, outgoing_node.index);

                return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
            } else {
                temp.insert(vec![
                    new_source_edge,
                    new_node,
                    new_target_edge
                ]);

                temp.attach(source_node.index, new_source_edge_index);
                temp.attach(source_node.index, new_node_index);
                temp.attach(new_node_index, new_target_edge_index);
                temp.attach(new_target_edge_index, outgoing_node.index);

                return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
            }
        } else if !node_collections::can_connect(collection.get_nodes(), source_node.index, target_node.index) {
            return None;
        } 

        let new_node = self.factory.new_node(collection.len(), *node_type);

        temp.insert(vec![new_node]);

        temp.attach(source_node_index, collection.len());
        temp.attach(collection.len(), target_node_index);
        temp.detach(source_node_index, target_node_index);

        return self.repair_insert(temp, collection.len(), source_node, target_node);
    }

    fn repair_insert(&self,
        mut collection: Graph<T>, 
        new_node_index: usize,
        source_node: &Node<T>,
        target_node: &Node<T>
    ) -> Option<Graph<T>>
    {
        for _ in 0..collection.get(new_node_index).unwrap().arity().unwrap() - 1 {
            let other_source_node = node_collections::random_source_node(collection.get_nodes());

            if node_collections::can_connect(collection.get_nodes(), other_source_node.index, new_node_index) {
                collection.attach(other_source_node.index, new_node_index);
            }
        }

        if !collection.is_valid() {
            return None;
        }

        return Some(collection.set_cycles(vec![source_node.index, target_node.index]));
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