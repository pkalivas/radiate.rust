
#[cfg(test)]
mod tests {

    use radiate_extensions::architects::architect::Architect;
    use radiate_extensions::architects::factories::node_factory::NodeFactory;
    use radiate_extensions::architects::node_collections::graph::Graph;
    use radiate_extensions::architects::node_collections::node_collection::NodeCollection;

    #[test]
    fn test_graph() {
        let factory = NodeFactory::new()
            .input_values(vec![1, 2, 3])
            .output_values(vec![4, 5, 6])
            .gate_values(vec![7, 8, 9])
            .aggregate_values(vec![10, 11, 12])
            .weight_values(vec![13, 14, 15]);

        let architect = Architect::<Graph<i32>, i32>::new(&factory);

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
        let factory = NodeFactory::<f32>::regression(2);
        let architect = Architect::<Graph<f32>, f32>::new(&factory);

        let graph = architect.weighted_cyclic(2, 2, 2);

        for node in graph.get_nodes() {
            println!("{:?}", node);
        }
    }
}