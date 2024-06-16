
#[derive(Debug, Default)]
pub struct GeneticEngineBuilder {
    pub population_size: i32,
}

impl GeneticEngineBuilder {

    pub fn new() -> Self {
        GeneticEngineBuilder {
            population_size: 100
        }
    }

}