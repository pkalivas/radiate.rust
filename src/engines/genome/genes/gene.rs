
pub trait Valid {
    fn is_valid(&self) -> bool { true }
}

pub trait Gene<G: Gene<G, A>, A>: Clone + PartialEq + Valid {
    fn allele(&self) -> A;
    fn new_instance(&self) -> G;
    fn from_allele(&self, allele: &A) -> G;
}

pub trait BoundGene<G: BoundGene<G, A>, A>: Gene<G, A> {
    fn upper_bound(&self) -> &A;
    fn lower_bound(&self) -> &A;
    fn with_bounds(self, upper_bound: A, lower_bound: A) -> G;
}

pub trait NumericGene<G: NumericGene<G, A>, A>: BoundGene<G, A> {
    fn add(&self, other: &G) -> G;
    fn sub(&self, other: &G) -> G;
    fn mul(&self, other: &G) -> G;
    fn div(&self, other: &G) -> G;
}

