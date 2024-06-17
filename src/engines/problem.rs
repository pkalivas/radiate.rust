use crate::engines::genome::genes::gene::Gene;
use crate::engines::score::Score;

pub trait Problem<TGene, T>
where
    TGene: Gene<TGene>,
{
    fn evaluate(&self, item: &T) -> Score;
}