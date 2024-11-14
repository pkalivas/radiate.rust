
use radiate_rust::engines::genome::*;
use radiate_rust::{Alterer, Crossover};

use crate::architects::node_collections::*;
use crate::operations::op::Ops;

pub struct TreeCrossover<T>
where
    T: Clone + PartialEq + Default
{
    pub rate: f32,
    pub max_height: usize,
    _marker: std::marker::PhantomData<T>
}

impl<T> TreeCrossover<T>
where
    T: Clone + PartialEq + Default + 'static
{
    pub fn alterer(max_height: usize, rate: f32) -> Alterer<Node<T>, Ops<T>> {
        let alterer = Self {
            rate,
            max_height,
            _marker: std::marker::PhantomData
        };

        Alterer::Crossover(Box::new(alterer))
    }

    fn can_cross(&self, one_nodes: &[Node<T>], two_nodes: &[Node<T>], one_index: usize, two_index: usize) -> bool {
        if one_index < 1 || two_index < 1 {
            return false;
        }

        let one_depth = self.depth(one_nodes, one_index);
        let two_depth = self.depth(two_nodes, two_index);

        let one_level = self.level(one_nodes, one_index);
        let two_level = self.level(two_nodes, two_index);

        one_depth + two_level <= self.max_height && two_depth + one_level <= self.max_height
    }

    fn depth(&self, nodes: &[Node<T>], index: usize) -> usize {
        nodes[index].outgoing
            .iter()
            .map(|outgoing| self.depth(nodes, *outgoing))
            .max()
            .unwrap_or_default() + 1
    }

    fn level(&self, nodes: &[Node<T>], index: usize) -> usize {
        nodes[index].incoming
            .iter()
            .map(|incoming| self.level(nodes, *incoming))
            .max()
            .unwrap_or_default() + 1
    }

    fn breadth_first_copy(&self, nodes: &[Node<T>], index: usize) -> Vec<Node<T>> {
        let mut queue = vec![index];
        let mut visited = vec![false; nodes.len()];
        let mut new_nodes = Vec::new();

        while !queue.is_empty() {
            let current = queue.remove(0);

            if visited[current] {
                continue;
            }

            visited[current] = true;

            let node = nodes[current].clone();
            new_nodes.push(node);

            for outgoing in nodes[current].outgoing.iter() {
                queue.push(*outgoing);
            }
        }

        new_nodes
    }

    fn swap(&self, one_index: usize, two_index: usize, one_nodes: &[Node<T>], two_nodes: &[Node<T>]) -> Vec<Node<T>> {
        let two_nodes_copy = self.breadth_first_copy(two_nodes, two_index);
        let one_nodes_copy = self.breadth_first_copy(one_nodes, one_index);
        let other_one_nodes = one_nodes
            .iter()
            .filter(|node| !one_nodes_copy.contains(node))
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>();

        let mut new_one_tree = Tree::from_nodes(other_one_nodes.iter().map(|node| node.clone()).collect::<Vec<Node<T>>>()).reindex(0);
        let mut two_sub_tree = Tree::from_nodes(two_nodes_copy).reindex(one_nodes.len());

        for incoming_node_index in one_nodes[one_index].incoming.iter() {
            let node = new_one_tree.get_mut(*incoming_node_index).unwrap();

            node.outgoing.remove(&one_index);
            node.outgoing.insert(two_sub_tree.get(0).unwrap().index);
            two_sub_tree.get_mut(0).unwrap().incoming.insert(node.index);
        }

        let mut result = new_one_tree
            .iter()
            .chain(two_sub_tree.iter())
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>();

        result.sort_by(|a, b| a.index.cmp(&b.index));

        let temp = Tree::from_nodes(result).reindex(0);

        return temp.get_nodes().iter().map(|node| node.clone()).collect::<Vec<Node<T>>>();
    
        // result
    }
}

impl<T> Crossover<Node<T>, Ops<T>> for TreeCrossover<T>
where 
    T: Clone + PartialEq + Default + 'static
{
    fn cross_rate(&self) -> f32 {
        self.rate
    }

    fn cross(&self,
        population: &mut Population<Node<T>, Ops<T>>,
        parent_indexes: &[usize],
        generation: i32) 
    {
        let parent_one = population.get(parent_indexes[0]);
        let parent_two = population.get(parent_indexes[1]);

        let geno_one = parent_one.genotype();
        let geno_two = parent_two.genotype();

        let chrom_one_index = rand::random::<usize>() % geno_one.len();
        let chrom_two_index = rand::random::<usize>() % geno_two.len();

        let chrom_one = geno_one.get_chromosome(chrom_one_index);
        let chrom_two = geno_two.get_chromosome(chrom_two_index);

        let swap_one_index = rand::random::<usize>() % chrom_one.len();
        let swap_two_index = rand::random::<usize>() % chrom_two.len();

        if !self.can_cross(chrom_one.get_genes(), chrom_two.get_genes(), swap_one_index, swap_two_index) {
            return;
        }

        let tree_one = self.swap(swap_one_index, swap_two_index, chrom_one.get_genes(), chrom_two.get_genes());
        let tree_two = self.swap(swap_two_index, swap_one_index, chrom_two.get_genes(), chrom_one.get_genes());

        let new_chrom_one = Chromosome::from_genes(tree_one);
        let new_chrom_two = Chromosome::from_genes(tree_two);

        let new_geno_one = Genotype::from_chromosomes(vec![new_chrom_one]);
        let new_geno_two = Genotype::from_chromosomes(vec![new_chrom_two]);

        population.set(parent_indexes[0], Phenotype::from_genotype(new_geno_one, generation));
        population.set(parent_indexes[1], Phenotype::from_genotype(new_geno_two, generation));
    }
}