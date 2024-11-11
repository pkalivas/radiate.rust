use std::sync::Arc;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::factories::op_factory::OpFactory;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::operations::op::Ops;


fn main() {

    let factory = Arc::new(OpFactory::<f32>::regression(2));
    let architect = Architect::<Graph<Ops<f32>>, Ops<f32>>::new(factory);

    let graph = architect.weighted_cyclic(2, 2, 2);

    for node in graph.get_nodes() {
        println!("{:?}", node);
    }
}
