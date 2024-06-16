
pub trait Allele<T> {
    fn allele(&self) -> &T;
}

pub trait Gene<TGene> : Clone + PartialEq
    where TGene: Gene<TGene>
{
    fn new_instance() -> TGene;
    fn from_gene(gene: &TGene) -> TGene;
}

pub trait NumericGene<TGene, T> : Gene<TGene> + Allele<T>
    where TGene: NumericGene<TGene, T>
{
    fn add(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn sub(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn mul(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn div(&self, other: &impl NumericGene<TGene, T>) -> TGene;
}