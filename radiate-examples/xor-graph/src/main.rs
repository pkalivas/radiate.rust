use radiate_rust::engines::codexes::Codex;

use radiate_extensions::architects::codexes::graph_codex::GraphCodex;
use radiate_extensions::architects::factories::node_factory::NodeFactory;


fn main() {
    let factory = NodeFactory::<f32>::regression(2);
    let graph_codex = GraphCodex::new(2, 2, factory)
        .set_nodes(|arc, _| arc.weighted_cyclic(2, 2, 2));

    let genotype = graph_codex.encode();

    for chromosome in genotype.iter() {
        for gene in chromosome.iter() {
            println!("{:?}", gene);
        }
    }
}
