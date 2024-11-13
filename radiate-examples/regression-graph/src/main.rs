
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
            Alterer::alterer(GraphCrossover::new(0.5, 0.5, 0.2)),
            Alterer::mutation(OpMutator::new(factory.clone(), 0.01, 0.05)),
            Alterer::alterer(GraphMutator::new(factory.clone())
                .add_mutation(NodeType::Weight, 0.05)
                .add_mutation(NodeType::Aggregate, 0.01)
                .add_mutation(NodeType::Gate, 0.03))])
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
    let mut reducer = GraphReducer::new(&result.best);
    for sample in get_sample_set().get_samples().iter() {
        let output = reducer.reduce(&sample.1);
        println!("{:.2?} :: {:.2?}", sample.2[0], output[0]);
    }
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





                   // Alterer::Alterer(Box::new(
            //     GraphCrossover::new(0.5, 0.5, 0.2)
            // )),
            // Alterer::Mutation(Box::new(
            //     OpMutator::new(factory.clone(), 0.01, 0.05)
            // )),
            // Alterer::Mutation(Box::new(
            //     GraphMutator::new(factory.clone())
            //         .add_mutation(NodeType::Weight, 0.03)
            //         .add_mutation(NodeType::Aggregate, 0.01)
            //         .add_mutation(NodeType::Gate, 0.05)
            // )),