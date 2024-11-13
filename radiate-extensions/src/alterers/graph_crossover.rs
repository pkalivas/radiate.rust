
use std::collections::HashMap;

use radiate_rust::engines::{alterers::{alter::Alter, crossovers::crossover::Crossover}, genome::{chromosome::Chromosome, genes::gene::Gene, genotype::Genotype, phenotype::Phenotype, population::Population}, optimize::Optimize};

use crate::{architects::node_collections::node::Node, operations::op::Ops};


const NUM_PARENTS: usize = 2;


pub struct GraphCrossover<T>
where
    T: Clone + PartialEq + Default
{
    pub crossover_rate: f32,
    pub crossover_parent_node_rate: f32,
    pub reenable_shared_node_rate: f32,
    _marker: std::marker::PhantomData<T>
}

impl<T> GraphCrossover<T>
where
    T: Clone + PartialEq + Default
{
    pub fn new(crossover_rate: f32, crossover_parent_node_rate: f32, reenable_shared_node_rate: f32) -> Self {
        Self {
            crossover_rate,
            crossover_parent_node_rate,
            reenable_shared_node_rate,
            _marker: std::marker::PhantomData
        }
    }

    pub fn cross(&self, population: &mut Population<Node<T>, Ops<T>>, indexes: &[usize], generation: i32) {
        let parent_one = population.get(indexes[0]);
        let parent_two = population.get(indexes[1]);

        let geno_one = parent_one.genotype();
        let geno_two = parent_two.genotype();

        let chromo_index = rand::random::<usize>() % std::cmp::min(geno_one.len(), geno_two.len());

        let chromo_one = geno_one.get_chromosome(chromo_index);
        let chromo_two = geno_two.get_chromosome(chromo_index);

        let one_lookup = GraphCrossover::<T>::chromosome_identity_lookup(chromo_one);
        let two_lookup = GraphCrossover::<T>::chromosome_identity_lookup(chromo_two);

        let mut new_chromo_one = chromo_one.clone();
        let mut num_crosses = 0;

        let matching_identity = one_lookup.keys().filter(|key| two_lookup.contains_key(key));

        for identity in matching_identity {
            
            let node_one = chromo_one.get_gene(*one_lookup.get(identity).unwrap());
            let node_two = chromo_two.get_gene(*two_lookup.get(identity).unwrap());

            if node_one.node_type != node_two.node_type || node_one.arity() != node_two.arity() {
                continue;
            }

            if (!node_one.enabled || !node_two.enabled) && rand::random::<f32>() < self.reenable_shared_node_rate {
                let mut new_gene = node_one.clone();
                new_gene.enabled = true;
                new_chromo_one.set_gene(*node_one.index(), new_gene);
                num_crosses += 1;
            }

            if rand::random::<f32>() < self.crossover_parent_node_rate {
                new_chromo_one.set_gene(*node_one.index(), node_one.from_allele(&node_two.allele()));
                num_crosses += 1;
            }
        }

        if num_crosses > 0 {
            let new_genotype_one = Genotype { chromosomes: vec![new_chromo_one] };
            let new_phenotype = Phenotype::from_genotype(new_genotype_one, generation);

            population.set(indexes[0], new_phenotype);
        }
    }

    pub fn distinct_subset(limit: usize) -> Vec<usize> {
        let mut subset = Vec::new();
        
        while subset.len() < NUM_PARENTS {
            let index = rand::random::<usize>() % limit;
            if !subset.contains(&index) {
                subset.push(index);
            }
        }

        subset.sort();
        subset
    }

    fn chromosome_identity_lookup(chromosome: &Chromosome<Node<T>, Ops<T>>) -> HashMap<u64, usize> {
        let mut lookup = HashMap::new();

        for (i, gene) in chromosome.iter().enumerate() {
            lookup.insert(GraphCrossover::<T>::node_identity(gene), i);
        }

        lookup
    }

    fn node_identity(node: &Node<T>) -> u64 {
        let mut result = GraphCrossover::<T>::cantor_pairing(u64::from(node.index as u64), u64::from(node.arity().unwrap_or(1)));

        for incoming in node.incoming.iter() {
            result = GraphCrossover::<T>::cantor_pairing(u64::from(*incoming as u64), result);
        }

        for outgoing in node.outgoing.iter() {
            result = GraphCrossover::<T>::cantor_pairing(u64::from(*outgoing as u64), result);
        }

        result
    }

    fn cantor_pairing(a: u64, b: u64) -> u64 {
        let added = a.wrapping_add(b);
        let second_added = added.wrapping_add(1);
        let multiplied = added.wrapping_mul(second_added);
        let divided = multiplied.wrapping_div(2);
        let result = divided.wrapping_add(b);

        result
    }
}

impl<T> Alter<Node<T>, Ops<T>> for GraphCrossover<T>
where 
    T: Clone + PartialEq + Default
{
    fn alter(&self, population: &mut Population<Node<T>, Ops<T>>, optimize: &Optimize, generation: i32) {
        optimize.sort(population);

        for _ in 0..population.len() {
            if rand::random::<f32>() < self.crossover_rate && population.len() > NUM_PARENTS {
                let parent_indexes = GraphCrossover::<T>::distinct_subset(population.len());
                self.cross(population, &parent_indexes, generation);
                
            } else {
                // TODO: Do we need to copy the current value??
                // population.set(i, population.get(i).clone());
            }
        }
    }
}


impl<T> Crossover<Node<T>, Ops<T>> for GraphCrossover<T> 
where
    T: Clone + PartialEq + Default
{
    fn cross_rate(&self) -> f32 {
        self.crossover_rate
    }

    fn cross_chromosomes(
        &self,
        chrom_one: &mut Chromosome<Node<T>, Ops<T>>,
        chrom_two: &mut Chromosome<Node<T>, Ops<T>>
    ) -> i32 {
        let mut cross_count = 0;

        let one_lookup = GraphCrossover::<T>::chromosome_identity_lookup(&chrom_one);
        let two_lookup = GraphCrossover::<T>::chromosome_identity_lookup(&chrom_two);

        let matching_identity = one_lookup.keys().filter(|key| two_lookup.contains_key(key));

        for identity in matching_identity {
            
            let node_one = chrom_one.get_gene(*one_lookup.get(identity).unwrap());
            let node_two = chrom_two.get_gene(*two_lookup.get(identity).unwrap());

            if node_one.node_type != node_two.node_type || node_one.arity() != node_two.arity() {
                continue;
            }

            if rand::random::<f32>() < self.crossover_parent_node_rate {
                chrom_one.set_gene(*node_one.index(), node_one.from_allele(&node_two.value));
                cross_count += 1;
            }
        }

        cross_count
    }
}
