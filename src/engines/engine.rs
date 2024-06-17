use super::genome::population::Population;
use super::genome::genes::gene::Gene;


pub trait Engine<TGene> 
    where TGene : Gene<TGene>
{
    fn run<T>(&self, limit: T) where T: Fn(&Population<TGene>) -> bool;
}