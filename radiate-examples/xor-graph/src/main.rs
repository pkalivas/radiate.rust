use std::sync::Arc;

use radiate_rust::engines::codexes::Codex;

use radiate_extensions::architects::codexes::graph_codex::GraphCodex;
use radiate_extensions::architects::factories::op_factory::OpFactory;


fn main() {
    let factory = Arc::new(OpFactory::<f32>::regression(2));
    let graph_codex = GraphCodex::new(2, 2, factory.clone())
        .set_nodes(|arc, _| arc.weighted_cyclic(2, 2, 2));

    let genotype = graph_codex.encode();

    for chromosome in genotype.iter() {
        for gene in chromosome.iter() {
            println!("{:?}", gene);
        }
    }
}
