use rand::random;
use radiate_rust::engines::{codex, genome::{genes::bit_gene::BitGene, genotype::Genotype}, problem::Problem, score::Score};

fn main() {

    let knapsack = Knapsack::new(10);
    // let codex = codex::subset(&knapsack.items);

    // let engine = GeneticEngine::from_codex(codex)
    //     .population_size(100)
    //     .minimizing()
    //     .offspring_selector(Selector::Elitism)
    //     .survivor_selector(Selector::Tournament(4))
    //     .alterer(vec![
    //         Alterer::Mutator(0.001),
    //         Alterer::UniformCrossover(0.5)
    //     ])
    //     .fitness_fn(|genotype: &Vec<Item>| {
    //         Score::from_int(0)
    //     })
    //     .build();
    println!("{:?}", knapsack);
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
}

impl<BitGene, bool, Knapsack> Problem<BitGene, bool, Knapsack> for Knapsack {
    fn evaluate(&self, genotype: &Genotype<BitGene, bool>) -> Score {
        let phenotype = self.decode(genotype);
        let mut weight = 0_f32;
        let mut value = 0_f32;

        for item in &phenotype {
            weight += item.weight;
            value += item.value;
        }

        if weight > self.capacity {
            Score::from_int(0)
        } else {
            Score::from_float(value)
        }
    }

    fn encode(&self) -> Genotype<BitGene, bool> {
        let mut genes = Vec::new();
        for _ in 0..self.size {
            genes.push(BitGene::new());
        }

        Genotype::from_genes(genes)
    }

    fn decode(&self, genotype: &Genotype<BitGene, bool>) -> Vec<Item> {
        let mut idx = 0;
        let mut result = Vec::new();
        for gene in genotype.chromosomes[0].iter() {
            if *gene.allele() {
                result.push(self.items[idx].clone());
            }
            idx += 1;
        }

        result
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
