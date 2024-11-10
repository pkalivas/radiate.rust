use std::vec;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::architects::node_factory::NodeFactory;
use radiate_extensions::architects::nodes::node_gene::NodeGene;


fn main() {

    let factory = NodeFactory::new()
        .inputs(vec![1, 2, 3])
        .outputs(vec![4, 5, 6])
        .gates(vec![7, 8, 9])
        .aggregates(vec![10, 11, 12])
        .weights(vec![13, 14, 15]);

    let architect = Architect::<Graph<NodeGene<i32>, i32>, NodeGene<i32>, i32>::new(factory);

    let graph = architect
        .build(|arc, builder| builder
            .one_to_one(&arc.input(2), &arc.output(2))
            .build());

    let nodes = graph.get_nodes();

    for node in nodes {
        println!("{:?}", node);
    }
}




    // let add_op = op::add();
    // let name = add_op.name();

    // let result = add_op.apply(&[1, 2]);

    // println!("{:?} Result: {}", add_op.name(), result);