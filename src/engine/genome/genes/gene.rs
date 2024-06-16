
pub trait Gene<TGene, T>
    where TGene : Gene<TGene, T> {

    fn allele() -> T;
    fn new_instance() -> TGene;
    fn from_value(value: T) -> TGene;
}