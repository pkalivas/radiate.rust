
pub trait Gene<TGene, T> : Clone + PartialEq
    where TGene: Gene<TGene, T>
{
    fn allele(&self) -> &T;
    fn new_instance() -> TGene;
    fn from_value(value: T) -> TGene;
}

