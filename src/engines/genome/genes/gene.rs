pub trait Allele<T> {
    fn allele(&self) -> &T;
}

pub trait Gene<TGene>: Clone + PartialEq
where
    TGene: Gene<TGene>,
{
    fn new_instance(&self) -> TGene;
    fn is_valid(&self) -> bool;
    fn from_gene(&self, gene: &TGene) -> TGene;
}

pub trait BoundGene<TGene, T>: Gene<TGene> + Allele<T>
where
    TGene: BoundGene<TGene, T>,
{
    fn upper_bound(&self) -> &T;
    fn lower_bound(&self) -> &T;
    fn with_bounds(self, upper_bound: T, lower_bound: T) -> TGene;
}

pub trait NumericGene<TGene, T>: BoundGene<TGene, T>
where
    TGene: NumericGene<TGene, T>,
{
    fn add(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn sub(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn mul(&self, other: &impl NumericGene<TGene, T>) -> TGene;
    fn div(&self, other: &impl NumericGene<TGene, T>) -> TGene;
}

