
#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use radiate_extensions::architects::architect::Architect;
    use radiate_extensions::architects::factories::op_factory::OpFactory;
    use radiate_extensions::architects::node_collections::graph::Graph;
    use radiate_extensions::architects::node_collections::node_collection::NodeCollection;
    use radiate_extensions::architects::factories::value_factory::ValueFactory;
    use radiate_extensions::operations::op::Ops;

    #[test]
    fn test_graph() {
        let factory = Arc::new(ValueFactory::new()
            .inputs(vec![1, 2, 3])
            .outputs(vec![4, 5, 6])
            .gates(vec![7, 8, 9])
            .aggregates(vec![10, 11, 12])
            .weights(vec![13, 14, 15]));

        let architect = Architect::<Graph<i32>, i32>::new(factory);

        let graph = architect
            .build(|arc, builder| builder
                .one_to_one(&arc.input(2), &arc.output(2))
                .build());

        let nodes = graph.get_nodes();

        for node in nodes {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_acyclic_graph() {
        let factory = Arc::new(OpFactory::<f32>::regression(2));
        let architect = Architect::<Graph<Ops<f32>>, Ops<f32>>::new(factory);

        let graph = architect.weighted_cyclic(2, 2, 2);

        for node in graph.get_nodes() {
            println!("{:?}", node);
        }
    }
}