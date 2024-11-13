use radiate_extensions::alterers::graph_crossover::GraphCrossover;
use radiate_extensions::alterers::graph_mutator::GraphMutator;
use radiate_extensions::alterers::op_mutator::OpMutator;
use radiate_extensions::architects::node_collections::graphs::graph::Graph;
use radiate_extensions::architects::node_collections::graphs::graph_reducer::GraphReducer;
use radiate_extensions::architects::schema::node_types::NodeType;
use radiate_extensions::operations::op;
use radiate_extensions::problems::error_functions::ErrorFunction;
use radiate_extensions::problems::regression::Regression;
use radiate_extensions::problems::sample_set::SampleSet;
use radiate_rust::engines::alterers::alter::Alterer;

use radiate_extensions::architects::node_collections::graphs::graph_codex::GraphCodex;
use radiate_extensions::architects::node_collections::node::Node;
use radiate_extensions::architects::node_collections::node_factory::NodeFactory;
use radiate_extensions::operations::op::Ops;
use radiate_rust::engines::engine_context::EngineContext;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::score::Score;

fn main() {
    let factory = NodeFactory::<f32>::regression(2)
        .outputs(vec![
            op::sigmoid()
        ]);

    let graph_codex = GraphCodex::from_shape(2, 1, &factory);

    let regression = Regression::new(get_sample_set(), ErrorFunction::MSE);

    let engine = GeneticEngine::from_codex(&graph_codex)
        .minimizing()
        .alterer(vec![
            Alterer::Alterer(Box::new(
                GraphCrossover::new(0.5, 0.5, 0.2)
            )),
            Alterer::Mutation(Box::new(
                OpMutator::new(factory.clone(), 0.001, 0.05)
            )),
            Alterer::Mutation(Box::new(
                GraphMutator::new(factory.clone())
                    .add_mutation(NodeType::Weight, 0.05)
                    .add_mutation(NodeType::Aggregate, 0.03)
                    .add_mutation(NodeType::Gate, 0.03)
            )),
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
        println!("{:?} -> epected: {:?}, actual: {:?}", sample.1, sample.2, output);
    }
}

fn get_sample_set() -> SampleSet<f32> {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![1.0, 1.0],
        vec![1.0, 0.0],
        vec![0.0, 1.0]
    ];

    let answers = vec![
        vec![0.0],
        vec![0.0],
        vec![1.0],
        vec![1.0]
    ];

    SampleSet::from_vecs(inputs, answers)
}


        // .set_nodes(|arc, conn| {
        //     conn.layer(vec![
        //         &arc.weighted_acyclic(2, 3),
        //         &arc.weighted_acyclic(3, 1)
        //     ])
        //     .build()
        //     // arc.weighted_cyclic(2, 1, 2)
        // });

