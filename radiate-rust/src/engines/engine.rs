use super::genome::genes::gene::Gene;
use crate::engines::engine_context::EngineContext;

pub trait Engine<G, A, T>
where
    G: Gene<G, A>,
    T: Clone,
{
    fn fit<F: Fn(&EngineContext<G, A, T>) -> bool>(&self, limit: F) -> EngineContext<G, A, T>;

    fn start(&self) -> EngineContext<G, A, T>;

    fn stop(&self, output: &mut EngineContext<G, A, T>) -> EngineContext<G, A, T> {
        output.timer.stop();
        output.clone()
    }
}
