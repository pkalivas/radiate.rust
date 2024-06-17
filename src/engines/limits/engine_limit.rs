
pub enum EngineLimit {
    Seconds(u64),
    Generations(u64),
    Fitness(f64),
}