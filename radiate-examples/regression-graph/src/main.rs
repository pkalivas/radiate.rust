
use radiate_extensions::*;
use radiate_rust::*;

fn main() {
    let factory = NodeFactory::<f32>::regression(1)
        .gates(vec![
            op::add(),
            op::sub(),
            op::mul(),
        ]);

    let graph_codex = GraphCodex::from_shape(1, 1, &factory);

    let regression = Regression::new(get_sample_set(), ErrorFunction::MSE);

    let engine = GeneticEngine::from_codex(&graph_codex)
        .minimizing()
        .offspring_selector(Selector::Boltzmann(4_f32))
        .alterer(vec![
            GraphCrossover::alterer(0.5, 0.5),
            OpMutator::alterer(factory.clone(), 0.01, 0.05),
            GraphMutator::alterer(factory.clone(), vec![
                NodeMutate::Forward(NodeType::Weight, 0.05),
                NodeMutate::Forward(NodeType::Aggregate, 0.03),
                NodeMutate::Forward(NodeType::Gate, 0.03),
            ])
        ])
        .fitness_fn(move |genotype: &Graph<f32>| {
            let mut reducer = GraphReducer::new(genotype);
            Score::from_f32(regression.error(|input| {
                reducer.reduce(&input)
            }))
        })
        .build();

    let result = engine.run(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.score().as_float());
        output.score().as_float() < 0.01 || output.seconds() > 5.0
    });

    display(&result);
}

fn display(result: &EngineContext<Node<f32>, Ops<f32>, Graph<f32>>) {
    for node in result.best.nodes.iter() {
        println!("{:?}", node);
    }
    println!("{:?}", result.timer.elapsed());

    let mut regression_accuracy = 0.0;
    let mut total = 0.0;

    let mut reducer = GraphReducer::new(&result.best);
    for sample in get_sample_set().get_samples().iter() {
        let output = reducer.reduce(&sample.1);

        total += sample.2[0].abs();
        regression_accuracy += (sample.2[0] - output[0]).abs();

        println!("{:.2?} :: {:.2?}", sample.2[0], output[0]);
    }

    regression_accuracy = (total - regression_accuracy) / total;

    println!("Accuracy: {:.2?}", regression_accuracy);
}

fn get_sample_set() -> SampleSet<f32> {
    let mut inputs = Vec::new();
    let mut answers = Vec::new();

    let mut input = -1.0;
    for _ in -10..10 {
        input += 0.1;
        inputs.push(vec![input]);
        answers.push(vec![compupute(input)]);
    }

    SampleSet::from_vecs(inputs, answers)
}

fn compupute(x: f32) -> f32 {
    return 4.0 * x.powf(3.0) - 3.0 * x.powf(2.0) + x;
}