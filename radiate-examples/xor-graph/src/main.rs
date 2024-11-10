use std::vec;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::architects::node_factory::NodeFactory;
use radiate_extensions::operations::op;
use radiate_extensions::operations::op::Ops;


fn main() {

    let factory = NodeFactory::new()
        .inputs(vec![
            op::var(0),
            op::var(1),
        ])
        .weights(vec![op::weight()])
        .outputs(vec![
            op::add()
        ]);

    let architect = Architect::<Graph<Ops<f32>>, Ops<f32>>::new(factory);

    let graph = architect
        .build(|arc, builder| builder
            .all_to_all(&arc.input(2), &arc.output(2))
            .build());

    let nodes = graph.get_nodes();

    for node in nodes {
        println!("{:?}", node);
    }
}
