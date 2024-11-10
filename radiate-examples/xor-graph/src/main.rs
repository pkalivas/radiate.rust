use std::vec;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::architects::node_factory::NodeFactory;
use radiate_extensions::architects::nodes::node_gene::NodeGene;
use radiate_extensions::operations::op;
use radiate_extensions::operations::ops::Ops;


fn main() {

    let factory = NodeFactory::new()
        .inputs(vec![
            op::var(0),
            op::var(1),
        ])
        .outputs(vec![
            op::add(),
            op::sub(),
        ]);

    // let architect = Architect::<Graph<NodeGene<i32>, i32>, NodeGene<i32>, i32>::new(factory);
    let architect = Architect::<Graph<NodeGene<Ops<f32>>, Ops<f32>>, NodeGene<Ops<f32>>, Ops<f32>>::new(factory);


    let graph = architect
        .build(|arc, builder| builder
            .one_to_one(&arc.input(2), &arc.output(2))
            .build());

    let nodes = graph.get_nodes();

    for node in nodes {
        println!("{:?}", node);
    }
}
