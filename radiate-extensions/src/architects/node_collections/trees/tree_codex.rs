use radiate_rust::engines::genome::genotype::Genotype;
use radiate_rust::engines::genome::chromosome::Chromosome;
use radiate_rust::engines::codexes::Codex;

use crate::architects::*;
use crate::operations::op::Ops;


pub struct TreeCodex<'a, T>
where 
    T: Clone + PartialEq + Default
{
    pub depth: usize,
    pub factory: &'a NodeFactory<T>,
    pub nodes: Vec<Node<T>>,
}

impl<'a, T> TreeCodex<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub fn from_shape(depth: usize, factory: &'a NodeFactory<T>) -> Self {
        Self {
            depth,
            factory,
            nodes: Architect::<Tree<T>, T>::new(&factory)
                .tree(depth)
                .iter()
                .map(|node| node.clone())
                .collect::<Vec<Node<T>>>()
        }
    }
}

impl<'a, T> Codex<Node<T>, Ops<T>, Tree<T>> for TreeCodex<'a, T>
where 
    T: Clone + PartialEq + Default
{
    fn encode(&self) -> Genotype<Node<T>, Ops<T>> {
        let chromosome = Chromosome::from_genes(self.nodes.clone());
        Genotype::from_chromosomes(vec![chromosome])
    }

    fn decode(&self, genotype: &Genotype<Node<T>, Ops<T>>) -> Tree<T> {
        let chromosome = genotype.chromosomes.first().unwrap();
        let nodes = chromosome.genes.clone();

        Tree::from_nodes(nodes)
    }
}