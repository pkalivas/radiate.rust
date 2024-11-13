
use radiate_rust::{engines::genome::{chromosome::Chromosome, genes::gene::Valid}, Alter, Optimize, Phenotype, Population};
use rand::{random, seq::SliceRandom};

use crate::{architects::{node_collections::{self, graphs::graph::Graph, node::Node, node_collection::NodeCollection, node_factory::NodeFactory}, schema::node_types::NodeType}, operations::op::Ops};


pub struct NodeMutate {
    pub rate: f32,
    pub node_type: NodeType,
    pub recurrent: bool,
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
        self.mutations.push(NodeMutate { rate, node_type, recurrent: false });
        self
    }
    
    pub fn add_recurrent_mutation(mut self, node_type: NodeType, rate: f32) -> Self {
        self.mutations.push(NodeMutate { rate, node_type, recurrent: true });
        self
    }

    #[inline]
    pub fn insert_forward_node(&self, collection: &[Node<T>], node_type: &NodeType) -> Option<Vec<Node<T>>> {
        let source_node = node_collections::random_source_node(collection);
        let target_node = node_collections::random_target_node(collection);
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
                let mut temp = Graph::from_nodes(collection
                    .iter()
                    .map(|node| node.clone())
                    .chain(vec![new_source_edge, new_node])
                    .collect::<Vec<Node<T>>>());

                temp.attach(source_node_index, new_node_index);
                temp.attach(new_node_index, new_source_edge_index);
                temp.attach(new_source_edge_index, outgoing_node.index);
                temp.detach(source_node_index, outgoing_node.index);

                return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
            } else {
                let mut temp = Graph::from_nodes(collection
                    .iter()
                    .map(|node| node.clone())
                    .chain(vec![new_source_edge, new_node, new_target_edge])
                    .collect::<Vec<Node<T>>>());

                temp.attach(source_node.index, new_source_edge_index);
                temp.attach(new_source_edge_index, new_node_index);
                temp.attach(new_node_index, new_target_edge_index);
                temp.attach(new_target_edge_index, outgoing_node.index);

                return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
            }
        } else if !node_collections::can_connect(collection, source_node.index, target_node.index) {
            return None;
        }

        let mut temp = Graph::from_nodes(collection
            .iter()
            .map(|node| node.clone())
            .chain(vec![self.factory.new_node(collection.len(), *node_type)])
            .collect::<Vec<Node<T>>>());

        temp.attach(source_node_index, collection.len());
        temp.attach(collection.len(), target_node_index);
        temp.detach(source_node_index, target_node_index);

        return self.repair_insert(temp, collection.len(), source_node, target_node);
    }

    pub fn insert_recurrent_node(&self, collection: &[Node<T>], node_type: &NodeType) -> Option<Vec<Node<T>>> {
        let source_node = node_collections::random_source_node(collection);
        let target_node = node_collections::random_target_node(collection);
        let source_node_index = source_node.index;
        let target_node_index = target_node.index;

        let new_source_edge_index = collection.len();
        let new_node_index = collection.len() + 1;
        let new_target_edge_index = collection.len() + 2;
        let recurrent_edge_index = collection.len() + 3;

        if source_node.node_type == NodeType::Weight && node_type != &NodeType::Weight {
            let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
            let outgoing_node = collection.get(*source_node.outgoing.iter().next().unwrap()).unwrap();

            let new_source_edge = self.factory.new_node(new_source_edge_index, source_node.node_type);
            let new_node = self.factory.new_node(new_node_index, *node_type);
            let new_target_edge = self.factory.new_node(new_target_edge_index, source_node.node_type);
            let recurrent_edge = self.factory.new_node(recurrent_edge_index, source_node.node_type);

            if node_collections::is_locked(outgoing_node) {
                let mut temp = Graph::from_nodes(collection
                    .iter()
                    .map(|node| node.clone())
                    .chain(vec![new_source_edge, new_node, new_target_edge])
                    .collect::<Vec<Node<T>>>());

                temp.attach(incoming_node.index, new_node_index);
                temp.attach(new_node_index, new_source_edge_index);
                temp.attach(new_source_edge_index, new_node_index);
                temp.attach(new_node_index, new_target_edge_index);
                temp.attach(new_target_edge_index, outgoing_node.index);
                temp.detach(incoming_node.index, outgoing_node.index);

                return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
            } else {
                if !source_node.is_recurrent() {
                    let mut temp = Graph::from_nodes(collection
                        .iter()
                        .map(|node| node.clone())
                        .chain(vec![new_source_edge, new_node, new_target_edge, recurrent_edge])
                        .collect::<Vec<Node<T>>>());

                    temp.attach(incoming_node.index, new_source_edge_index);
                    temp.attach(new_source_edge_index, new_node_index);
                    temp.attach(new_node_index, new_target_edge_index);
                    temp.attach(new_target_edge_index, outgoing_node.index);
                    temp.attach(recurrent_edge_index, new_node_index);
                    temp.attach(new_node_index, recurrent_edge_index);

                    return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
                } else {
                    let mut temp = Graph::from_nodes(collection
                        .iter()
                        .map(|node| node.clone())
                        .chain(vec![new_source_edge, new_node, new_target_edge])
                        .collect::<Vec<Node<T>>>());
    
                    temp.attach(incoming_node.index, new_source_edge_index);
                    temp.attach(new_source_edge_index, new_node_index);
                    temp.attach(new_node_index, new_target_edge_index);
                    temp.attach(new_target_edge_index, outgoing_node.index);
    
                    return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
                }
            }
        } else if !node_collections::can_connect(collection, source_node.index, target_node.index) {
            return None;
        }

        let mut temp = Graph::from_nodes(collection
            .iter()
            .map(|node| node.clone())
            .chain(vec![self.factory.new_node(collection.len(), *node_type)])
            .collect::<Vec<Node<T>>>());

        temp.attach(source_node_index, collection.len());
        temp.attach(collection.len(), target_node_index);
        temp.detach(source_node_index, target_node_index);

        return self.repair_insert(temp, collection.len(), source_node, target_node);
    }


    #[inline]
    fn repair_insert(&self,
        mut collection: Graph<T>, 
        new_node_index: usize,
        source_node: &Node<T>,
        target_node: &Node<T>
    ) -> Option<Vec<Node<T>>>
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

        return Some(collection.set_cycles(vec![source_node.index, target_node.index]).into_iter().collect::<Vec<Node<T>>>());
    }
}

impl<T> Alter<Node<T>, Ops<T>> for GraphMutator<T>
where
    T: Clone + PartialEq + Default    
{
    fn alter(&self, population: &mut Population<Node<T>, Ops<T>>, _: &Optimize, generation: i32) {
        let mut rng = rand::thread_rng();

        for i in 0..population.len() {
            let mutation = self.mutations.choose(&mut rng).unwrap();

            if random::<f32>() > mutation.rate {
                continue;
            }

            let genotype = population.get(i).genotype();
            let chromosome_index = rand::random::<usize>() % genotype.len();
            let chromosome = genotype.get_chromosome(chromosome_index);

            let mutated_graph = if mutation.recurrent {
                self.insert_recurrent_node(&chromosome.genes, &mutation.node_type)
            } else {
                self.insert_forward_node(&chromosome.genes, &mutation.node_type)
            };

            if let Some(mutated_graph) = mutated_graph {
                if !mutated_graph.iter().all(|node| node.is_valid()) {
                    continue;
                }

                if mutated_graph.len() == chromosome.genes.len() {
                    continue;
                }

                let mut copied_genotype = genotype.clone();

                copied_genotype.set_chromosome(chromosome_index, Chromosome::from_genes(mutated_graph));
                population.set(i, Phenotype::from_genotype(copied_genotype, generation));
            }
        }
    }
}






// pub fn insert_recurrent_node(&self, collection: &[Node<T>], node_type: &NodeType) -> Option<Vec<Node<T>>> {
//     let source_node = node_collections::random_source_node(collection);
//     let target_node = node_collections::random_target_node(collection);
//     let source_node_index = source_node.index;
//     let target_node_index = target_node.index;

//     let new_source_edge_index = collection.len();
//     let new_node_index = collection.len() + 1;
//     let new_target_edge_index = collection.len() + 2;
//     let recurrent_edge_index = collection.len() + 3;

//     if source_node.node_type == NodeType::Weight && node_type != &NodeType::Weight {
//         let incoming_node = collection.get(*source_node.incoming.iter().next().unwrap()).unwrap();
//         let outgoing_node = collection.get(*source_node.outgoing.iter().next().unwrap()).unwrap();

//         let new_source_edge = self.factory.new_node(new_source_edge_index, source_node.node_type);
//         let new_node = self.factory.new_node(new_node_index, *node_type);
//         let new_target_edge = self.factory.new_node(new_target_edge_index, source_node.node_type);
//         let recurrent_edge = self.factory.new_node(recurrent_edge_index, source_node.node_type);

//         if node_collections::is_locked(outgoing_node) {
//             let mut temp = Graph::from_nodes(collection
//                 .iter()
//                 .map(|node| node.clone())
//                 .chain(vec![new_source_edge, new_node])
//                 .collect::<Vec<Node<T>>>());

//             temp.attach(source_node_index, new_node_index);
//             temp.attach(new_node_index, new_source_edge_index);
//             temp.attach(new_source_edge_index, new_node_index);
//             temp.attach(new_node_index, new_target_edge_index);
//             temp.attach(new_target_edge_index, outgoing_node.index);
//             temp.detach(source_node_index, outgoing_node.index);

//             return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
//         } else {
//             if !source_node.is_recurrent() {
                
//             }
//             let mut temp = Graph::from_nodes(collection
//                 .iter()
//                 .map(|node| node.clone())
//                 .chain(vec![new_source_edge, new_node, new_target_edge])
//                 .collect::<Vec<Node<T>>>());

//             temp.attach(source_node.index, new_source_edge_index);
//             temp.attach(new_source_edge_index, new_node_index);
//             temp.attach(new_node_index, new_target_edge_index);
//             temp.attach(new_target_edge_index, outgoing_node.index);

//             return self.repair_insert(temp, new_node_index, incoming_node, outgoing_node);
//         }
//     } else if !node_collections::can_connect(collection, source_node.index, target_node.index) {
//         return None;
//     }

//     let mut temp = Graph::from_nodes(collection
//         .iter()
//         .map(|node| node.clone())
//         .chain(vec![self.factory.new_node(collection.len(), *node_type)])
//         .collect::<Vec<Node<T>>>());

//     temp.attach(source_node_index, collection.len());
//     temp.attach(collection.len(), target_node_index);
//     temp.detach(source_node_index, target_node_index);

//     return self.repair_insert(temp, collection.len(), source_node, target_node);
// }
