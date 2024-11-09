use std::sync::LazyLock;

use rand::random;

use radiate_rust::engines::alterers::alter::Alterer;
use radiate_rust::engines::codexes::subset_codex::SubSetCodex;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::score::Score;
use radiate_rust::engines::selectors::selector::Selector;
use radiate_rust::engines::engine::Engine;


static KNAPSACK: LazyLock<Knapsack> = LazyLock::new(|| Knapsack::new(10));


fn main() {
    println!("Knapsack Capacity=[ {:?} ]", KNAPSACK.capacity);
    
    let codex = SubSetCodex::new(&KNAPSACK.items);

    let engine = GeneticEngine::from_codex(codex)
        .population_size(100)
        .maximizing()
        .offspring_selector(Selector::Elitism)
        .survivor_selector(Selector::Tournament(4))
        .alterer(vec![
            Alterer::Mutator(0.001),
            Alterer::UniformCrossover(0.5)
        ])
        .fitness_fn(move |genotype: &Vec<&Item>| KNAPSACK.fitness(genotype))
        .build();

    let result = engine.fit(|output| { 
        println!("[ {:?} ]: {:?}", output.index, output.score());
        output.index == 100
    });

    println!("{:?}", result);
}

pub struct Knapsack {
    pub capacity: f32,
    pub size: usize,
    pub items: Vec<Item>,
}

impl Knapsack {
    pub fn new(size: usize) -> Self {
        let items = Item::random_collection(size);
        Knapsack { capacity: size as f32 * 100_f32 / 3_f32, size, items }
    }

    pub fn fitness(&self, genotype: &Vec<&Item>) -> Score {
        let mut sum = 0_f32;
        let mut weight = 0_f32;
        for item in genotype {
            sum += item.value;
            weight += item.weight;
        }

        if weight > self.capacity {
            Score::from_f32(0_f32)
        } else {
            Score::from_f32(sum)
        }
    }
}

impl std::fmt::Debug for Knapsack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sum = 0_f32;
        for item in &self.items {
            sum += item.value;
        }

        write!(f, "Knapsack[capacity={:.2}, size={:.2}, sum={:.2}]", self.capacity, self.size, sum)
    }
}


#[derive(Debug, Clone)]
pub struct Item {
    pub weight: f32,
    pub value: f32,
}

impl Item {
    pub fn new(weight: f32, value: f32) -> Self {
        Item { weight, value }
    }

    pub fn random_collection(size: usize) -> Vec<Item> {
        (0..size)
            .map(|_| Item::new(random::<f32>() * 100.0, random::<f32>() * 100.0))
            .collect()
    }
}
