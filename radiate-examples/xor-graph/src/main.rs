use radiate_extensions::alterers::node_crossover::NodeCrossover;
use radiate_extensions::alterers::node_mutator::NodeMutator;
use radiate_extensions::architects::node_collections::graph::Graph;
use radiate_extensions::architects::node_collections::graph_reducer::GraphReducer;
use radiate_extensions::operations::op;
use radiate_extensions::problems::error_functions::ErrorFunction;
use radiate_extensions::problems::regression::Regression;
use radiate_extensions::problems::sample_set::SampleSet;
use radiate_rust::engines::alterers::alter::Alterer;

use radiate_extensions::architects::codexes::graph_codex::GraphCodex;
use radiate_extensions::architects::factories::node_factory::NodeFactory;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::score::Score;


fn main() {
    let factory = NodeFactory::<f32>::regression(2)
        .outputs(vec![
            op::sigmoid()
        ]);

    let graph_codex = GraphCodex::from_factory(factory)
        .set_nodes(|arc, conn| {
            conn.layer(vec![
                &arc.weighted_acyclic(2, 3),
                &arc.weighted_acyclic(3, 5),
                &arc.weighted_acyclic(5, 3),
                &arc.weighted_acyclic(3, 1)
            ])
            .build()
        });

    let regression = Regression::new(get_sample_set(), ErrorFunction::MSE);

    let engine = GeneticEngine::from_codex(&graph_codex)
        .minimizing()
        .alterer(vec![
            Alterer::Mutation(Box::new(
                NodeMutator::new(0.01, 0.05)
            )),
            Alterer::Crossover(Box::new(
                NodeCrossover::new(0.5)
            ))
        ])
        .fitness_fn(move |genotype: &Graph<f32>| {
            let mut reducer = GraphReducer::new(genotype);
            Score::from_f32(regression.error(|sample| {
                reducer.reduce(&sample)
            }))
        })
        .build();

    let result = engine.run(|output| {
        println!("[ {:?} ]: {:?}", output.index, output.score().as_float());
        output.index == 500
    });

    let mut reducer = GraphReducer::new(&result.best);
    for sample in get_sample_set().get_samples().iter() {
        let output = reducer.reduce(&sample.1);
        println!("{:?} -> epected: {:?}, actual: {:?}", sample.1, sample.2, output);
    }
}

pub fn get_sample_set() -> SampleSet<f32> {
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
