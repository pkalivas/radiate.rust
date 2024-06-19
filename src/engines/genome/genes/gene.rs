pub trait Allele<T> {
    fn allele(&self) -> &T;
}

pub trait Gene<TGene: Gene<TGene>>: Clone + PartialEq {
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

// pub trait NumericGene<TGene: Gene<TGene>, T>: NumericGene<TGene> {
//     fn add(&self, other: &impl NumericGene<TGene, T>) -> TGene;
//     fn sub(&self, other: &impl NumericGene<TGene, T>) -> TGene;
//     fn mul(&self, other: &impl NumericGene<TGene, T>) -> TGene;
//     fn div(&self, other: &impl NumericGene<TGene, T>) -> TGene;
// }


pub trait NumericGene<TGene>: Gene<TGene> + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div
where 
    TGene: NumericGene<TGene> { }