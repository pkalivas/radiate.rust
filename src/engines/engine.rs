use super::genome::genes::gene::Gene;
use crate::engines::engine_handle::EngineHandle;

pub trait Engine<G: Gene<G, A>, A, T: Clone> {
    fn fit<F: Fn(&EngineHandle<G, A, T>) -> bool>(&self, limit: F) -> EngineHandle<G, A, T>;

    fn start(&self) -> EngineHandle<G, A, T>;

    fn stop(&self, output: &mut EngineHandle<G, A, T>) -> EngineHandle<G, A, T> {
        output.timer.stop();
        output.clone()
    }
}
