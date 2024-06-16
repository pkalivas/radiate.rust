
#[derive(Debug)]
pub struct Genotype<'a, TGene, T>
    where TGene: Gene<TGene, T>
{
    pub chromosomes: Vec<Chromosome<'a, TGene, T>>
}