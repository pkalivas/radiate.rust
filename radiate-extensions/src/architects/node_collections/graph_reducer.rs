use crate::architects::nodes::tracer::Tracer;
use crate::architects::schema::node_types::NodeType;

use super::graph::Graph;
use super::node_collection::NodeCollection;


const CHECKS_WITHOUT_PROGRESS: i32 = 5000;


pub struct GraphReducer<T>
where
    T: Clone + PartialEq + Default
{
    pub graph: Graph<T>,
    pub tracers: Vec<Tracer<T>>,
}

impl<T> GraphReducer<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: Graph<T>) -> GraphReducer<T> {
        let tracers = graph
            .iter()
            .map(|node| Tracer::new(node.input_size()))
            .collect::<Vec<Tracer<T>>>();

        GraphReducer { graph, tracers }
    }

    pub fn reduce(&mut self, inputs: &[T]) -> Vec<T> {
        let mut checks = 0;
        let mut completed = vec![false; self.graph.len()];
        let mut result = Vec::new();

        let mut pending_index = 0;
        while pending_index < self.graph.len() {
            if checks > CHECKS_WITHOUT_PROGRESS {
                panic!("Failed to reduce graph.");
            }

            if let Some(node) = self.graph.get(pending_index) {
                if completed[node.index] {
                    continue;
                }

                if node.node_type == NodeType::Input {
                    self.tracers[node.index].args[node.index] = inputs[node.index].clone();
                } else {
                    let mut degree = node.incoming.len();
                    for incoming in &node.incoming {
                        if completed[*incoming] || self.graph.get(*incoming).unwrap().is_recurrent() {
                            degree -= 1;
                            self.tracers[node.index].args[degree] = self.tracers[*incoming].result.clone().unwrap();
                        }
                    }
    
                    if degree == 0 {
                        completed[node.index] = true;
                        self.tracers[node.index].activate(&node);
                        if node.node_type == NodeType::Output {
                            result.push(self.tracers[node.index].result.clone().unwrap());
                        }
                    } else {
                        pending_index = self.graph.len();
                    }
                }
            }
        }

        result
    }
}

// public static IEnumerable<TNode> Iterate<TCollection, TNode, T>(INodeCollection<TCollection, TNode, T> collection)
//         where TCollection : INodeCollection<TCollection, TNode, T>
//         where TNode : INode<TNode, T>
//     {
//         if (collection.CollectionType is not CollectionTypes.Graph)
//         {
//             throw new InvalidOperationException($"{nameof(GraphIterator)} Collection is not a graph.");
//         }
        
//         var checksWithoutProgress = 0;
//         var size = collection.Count();
//         var completed = new HashSet<int>(size);

//         var pendingIndex = 0;
//         while (pendingIndex < size)
//         {
//             if (checksWithoutProgress > MaxChecksWithoutProgress)
//             {
//                 throw new InvalidOperationException("Failed to iterate graph.");
//             }

//             var minPendingIndex = size;
//             for (var i = pendingIndex; i < size; i++)
//             {
//                 var node = collection[i];
                
//                 if (completed.Contains(node.Index))
//                 {
//                     continue;
//                 }
                
//                 var degree = node.Incoming.Count;
//                 foreach (var incoming in node.Incoming)
//                 {
//                     if (completed.Contains(incoming) || collection[incoming].IsRecurrent())
//                     {
//                         degree--;
//                     }
//                 }

//                 if (degree == 0)
//                 {
//                     completed.Add(node.Index);
//                     yield return node;
//                 }
//                 else
//                 {
//                     minPendingIndex = Math.Min(minPendingIndex, node.Index);
//                 }
//             }
            
//             checksWithoutProgress = minPendingIndex == pendingIndex ? checksWithoutProgress + 1 : 0;
//             pendingIndex = minPendingIndex;
//         }
//     }