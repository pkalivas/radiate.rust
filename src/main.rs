mod engine;

use engine::genome::genotype::Genotype;
use engine::genome::genes::float_gene::FloatGene;
use engine::genome::genes::char_gene::CharGene;
use engine::genome::chromosome::Chromosome;
use engine::genome::phenotype::Phenotype;
use engine::genome::population::Population;

fn main() {
    let float_genotype = create_float_genotype(3, 2);
    let char_genotype = create_char_genotype(3, 2);

    println!("{:?}", float_genotype);
    println!("{:?}", char_genotype);

    let population = Population::from_func(10, || {
        let genotype = create_char_genotype(3, 2);
        return Phenotype::from_genotype(genotype);
    });

    println!("{:?}", population);
}

fn create_char_genotype(gene_count: i32, chromosome_count: i32) -> Genotype<CharGene, char> {
    let chromosomes = (0..chromosome_count)
        .into_iter()
        .map(|_| {
            let genes = (0..gene_count)
                .into_iter()
                .map(|_| CharGene::new())
                .collect::<Vec<CharGene>>();

            return Chromosome::from_vec(genes);
        })
        .collect::<Vec<Chromosome<CharGene, char>>>();

    return Genotype::from_vec(chromosomes);
}

fn create_float_genotype(gene_count: i32, chromosome_count: i32) -> Genotype<FloatGene, f32> {
    let chromosomes = (0..chromosome_count)
        .into_iter()
        .map(|_| {
            let genes = (0..gene_count)
                .into_iter()
                .map(|_| FloatGene::new())
                .collect::<Vec<FloatGene>>();

            return Chromosome::from_vec(genes);
        })
        .collect::<Vec<Chromosome<FloatGene, f32>>>();

    return Genotype::from_vec(chromosomes);
}