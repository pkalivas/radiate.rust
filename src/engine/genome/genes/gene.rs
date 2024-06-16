
pub trait Allele<T> {
    fn allele(&self) -> &T;
}

pub trait Gene<TGene> : Clone + PartialEq
    where TGene: Gene<TGene>
{
    fn new_instance() -> TGene;
    fn from_gene(gene: TGene) -> TGene;
}

