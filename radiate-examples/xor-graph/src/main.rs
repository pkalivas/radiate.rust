use std::sync::Arc;
use std::vec;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::factories::op_factory::OpFactory;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::operations::op;
use radiate_extensions::operations::op::Ops;


fn main() {

    let factory = Arc::new(OpFactory::new()
        .inputs(vec![
            op::var(0),
            op::var(1),
            op::var(2), 
        ])
        .weights(vec![op::weight()])
        .outputs(vec![op::add()]));

    let architect = Architect::<Graph<Ops<f32>>, Ops<f32>>::new(factory);

    let graph = architect.weighted_acyclic(3, 2);
    // let graph = architect
    //     .build(|arc, builder| builder
    //         .all_to_all(&arc.input(2), &arc.output(2))
    //         .build());

    let nodes = graph.get_nodes();

    for node in nodes {
        println!("{:?}", node);
    }
}
