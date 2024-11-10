use std::vec;

use radiate_extensions::architects::architect::Architect;
use radiate_extensions::architects::node_collection_builder::NodeCollectionBuilder;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
use radiate_extensions::architects::node_factory::NodeFactory;
use radiate_extensions::architects::node_types::NodeType;
use radiate_extensions::architects::nodes::node_gene::NodeGene;
use radiate_extensions::operations::op;
use radiate_extensions::operations::op::Op;


fn main() {

    let mut factory = NodeFactory::new();

    factory.add_node_values(NodeType::Input, vec![1, 2, 3]);
    factory.add_node_values(NodeType::Output, vec![4, 5, 6]);
    factory.add_node_values(NodeType::Gate, vec![7, 8, 9]);
    factory.add_node_values(NodeType::Aggregate, vec![10, 11, 12]);
    factory.add_node_values(NodeType::Weight, vec![13, 14, 15]);

    let architect = Architect::<Graph<NodeGene<i32>, i32>, NodeGene<i32>, i32>::new(factory);

    let graph = architect
        .build(|arc, builder| builder
            .one_to_one(&arc.input(2), &arc.output(2))
            .build());

    let nodes = graph.get_nodes();

    for node in nodes {
        println!("{:?}", node);
    }

    let t = "";
}




    // let graph = architect.build(|builder: NodeCollectionBuilder<Graph<NodeGene<i32>, i32>, NodeGene<i32>, i32>| {
    //     builder.one_to_one(builder.input(3), builder.output(3)).build()
    // });


    // let input = factory.new_node::<NodeGene<i32>>(0, NodeType::Input);

    // let add_op = op::add();
    // let name = add_op.name();

    // let result = add_op.apply(&[1, 2]);

    // println!("{:?} Result: {}", add_op.name(), result);