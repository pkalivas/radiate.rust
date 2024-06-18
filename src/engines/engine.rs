use crate::engines::engine_handle::EngineHandle;
use super::genome::genes::gene::Gene;

pub trait Engine<TGene: Gene<TGene>, T: Clone> {
    fn fit<F: Fn(&EngineHandle<TGene, T>) -> bool>(&self, limit: F) -> EngineHandle<TGene, T>;
    fn start(&self) -> EngineHandle<TGene, T>;
    fn stop(&self, output: &mut EngineHandle<TGene, T>) -> EngineHandle<TGene, T> {
        output.timer.stop();
        output.clone()
    }
}