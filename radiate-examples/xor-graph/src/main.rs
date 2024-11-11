use radiate_extensions::architects::node_collections::graph_reducer::GraphReducer;
use radiate_extensions::problems::error_functions::ErrorFunction;
use radiate_extensions::problems::regression::Regression;
use radiate_extensions::problems::sample_set::SampleSet;
use radiate_rust::engines::codexes::Codex;

use radiate_extensions::architects::codexes::graph_codex::GraphCodex;
use radiate_extensions::architects::factories::node_factory::NodeFactory;


fn main() {
    let factory = NodeFactory::<f32>::regression(2);
    let graph_codex = GraphCodex::new(2, 2, factory)
        .set_nodes(|arc, _| arc.weighted_acyclic(2, 2));

    let sample_set = get_sample_set();
    let _ = Regression::new(sample_set, ErrorFunction::MSE);

    let genotype = graph_codex.encode();
    let decoded = graph_codex.decode(&genotype);

    let inputs = vec![1.0, 2.0];
    let mut reducer = GraphReducer::new(decoded);
    let outputs = reducer.reduce(&inputs);

    println!("{:?}", outputs);

    for chromosome in genotype.iter() {
        for gene in chromosome.iter() {
            println!("{:?}", gene);
        }
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
