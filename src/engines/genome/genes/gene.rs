
pub trait Valid {
    fn is_valid(&self) -> bool { true }
}

pub trait Gene<TGene: Gene<TGene, TAllele>, TAllele>: Clone + PartialEq + Valid {
    fn allele(&self) -> TAllele;
    fn new_instance(&self) -> TGene;
    fn from_gene(&self, gene: &TGene) -> TGene;
}

pub trait BoundGene<TGene: BoundGene<TGene, TAllele>, TAllele>: Gene<TGene, TAllele> {
    fn upper_bound(&self) -> &TAllele;
    fn lower_bound(&self) -> &TAllele;
    fn with_bounds(self, upper_bound: TAllele, lower_bound: TAllele) -> TGene;
}

pub trait NumericGene<TGene: NumericGene<TGene, TAllele>, TAllele>: BoundGene<TGene, TAllele> {
    fn add(&self, other: &TGene) -> TGene;
    fn sub(&self, other: &TGene) -> TGene;
    fn mul(&self, other: &TGene) -> TGene;
    fn div(&self, other: &TGene) -> TGene;
}

