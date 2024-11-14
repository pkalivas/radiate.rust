use radiate_rust::*;
use radiate_extensions::*;

fn main() {
    let factory = NodeFactory::<f32>::regression(2)
        .outputs(vec![
            op::sigmoid()
        ]);

    let graph_codex = GraphCodex::from_shape(1, 1, &factory)
        .set_nodes(|arc, _| arc.gru(1, 1, 1));

    let regression = Regression::new(get_sample_set(), ErrorFunction::MSE);

    let engine = GeneticEngine::from_codex(&graph_codex)
        .minimizing()
        .offspring_selector(Selector::Boltzmann(4_f32))
        .alterer(vec![
            GraphCrossover::alterer(0.5, 0.5),
            OpMutator::alterer(factory.clone(), 0.01, 0.05),
            GraphMutator::alterer(factory.clone(), vec![
                NodeMutate::Recurrent(NodeType::Weight, 0.05),
                NodeMutate::Recurrent(NodeType::Aggregate, 0.03),
                NodeMutate::Recurrent(NodeType::Gate, 0.03),
            ])
        ])
        .fitness_fn(move |genotype: &Graph<f32>| {
            let mut reducer = GraphReducer::new(genotype);
            Score::from_f32(regression.error(|input| reducer.reduce(&input)))
        })
        .build();

    let result = engine.run(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.score().as_float());
        output.index == 500 || output.score().as_float() < 0.01
    });

    display(&result);
}

fn display(result: &EngineContext<Node<f32>, Ops<f32>, Graph<f32>>) {
    for node in result.best.nodes.iter() {
        println!("{:?}", node);
    }
    println!("{:?}", result.timer.elapsed());
    let mut reducer = GraphReducer::new(&result.best);
    for sample in get_sample_set().get_samples().iter() {
        let output = reducer.reduce(&sample.1);
        println!("{:?} -> epected: {:?}, actual: {:.3?}", sample.1, sample.2, output);
    }
}

fn get_sample_set() -> SampleSet<f32> {
    let inputs = vec![
        vec![0.0],
        vec![0.0],
        vec![0.0],
        vec![1.0],
        vec![0.0],
        vec![0.0],
        vec![0.0]
    ];

    let answers = vec![
        vec![0.0],
        vec![0.0],
        vec![1.0],
        vec![0.0],
        vec![0.0],
        vec![0.0],
        vec![1.0]
    ];

    SampleSet::from_vecs(inputs, answers)
}
