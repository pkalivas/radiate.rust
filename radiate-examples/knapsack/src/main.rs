use rand::random;
use radiate_rust::engines::{codex, genome::genes::bit_gene::BitGene, problem::Problem};
use radiate_rust::engines::alterers::alter::Alterer;
use radiate_rust::engines::codex::Codex;
use radiate_rust::engines::genetic_engine::GeneticEngine;
use radiate_rust::engines::genome::genotype::Genotype;
use radiate_rust::engines::score::Score;
use radiate_rust::engines::selectors::selector::Selector;
use radiate_rust::engines::engine::Engine;

fn main() {

    let knapsack = Knapsack::new(10);
    // let codex = codex::subset(&knapsack.items);

    let engine = GeneticEngine::from_problem(knapsack)
        .population_size(100)
        .minimizing()
        .offspring_selector(Selector::Elitism)
        .survivor_selector(Selector::Tournament(4))
        .alterer(vec![
            Alterer::Mutator(0.001),
            Alterer::UniformCrossover(0.5)
        ])
        .build();

    let result = engine.fit(|output| {
        output.index % 100 == 0
    });

    let t = "";
    // println!("{:?}", result);
    
}

pub struct Knapsack {
    pub capacity: f32,
    pub size: usize,
    pub items: Vec<Item>,
    pub codex: Codex<BitGene, bool, Vec<i32>>,
}

impl Knapsack {
    pub fn new(size: usize) -> Self {
        let items = Item::random_collection(size);
        Knapsack {
            capacity: size as f32 * 100_f32 / 3_f32,
            size,
            items,
            codex: codex::subset_indices(size),
        }
    }
}

impl Problem<BitGene, bool, Vec<i32>> for Knapsack {
    fn evaluate(&self, genotype: &Genotype<BitGene, bool>) -> Score {
        let indices = self.codex.decode(genotype);
        let mut weight = 0_f32;
        let mut value = 0_f32;

        for idx in indices {
            let item = &self.items[idx as usize];
            weight += item.weight;
            value += item.value;
        }

        if weight > self.capacity {
            Score::from_int(0)
        } else {
            Score::from_float(value)
        }
    }

    fn codex(&self) -> &Codex<BitGene, bool, Vec<i32>> {
        &self.codex
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
