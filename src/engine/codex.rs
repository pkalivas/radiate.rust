use crate::engine::genome::genes::gene::Gene;
use crate::engine::genome::genotype::Genotype;

pub struct Codex<TGene, T>
    where TGene : Gene<TGene>
{
    pub encoder: Option<Box<dyn Fn() -> Genotype<TGene>>>,
    pub decoder: Option<fn(&Genotype<TGene>) -> T>
}

impl<TGene, T> Codex<TGene, T>
    where TGene : Gene<TGene>
{
    pub fn new() -> Self {
        Codex {
            encoder: None,
            decoder: None
        }
    }

    pub fn encode(&self) -> Genotype<TGene> {
        match &self.encoder {
            Some(encoder) => encoder(),
            None => panic!("Encoder not set")
        }
    }

    pub fn decode(&self, genotype: &Genotype<TGene>) -> T {
        match &self.decoder {
            Some(decoder) => decoder(genotype),
            None => panic!("Decoder not set")
        }
    }

    pub fn spawn(&self, num: i32) -> Vec<T> {
        (0..num).into_iter().map(|_| {
            self.decode(&self.encode())
        }).collect::<Vec<T>>()
    } 

    pub fn spawn_genotypes(&self, num: i32) -> Vec<Genotype<TGene>> {
        (0..num).into_iter().map(|_| {
            self.encode()
        }).collect::<Vec<Genotype<TGene>>>()
    }

    pub fn encoder(mut self, encoder: impl Fn() -> Genotype<TGene> + 'static) -> Self {
        self.encoder = Some(Box::new(encoder));
        self
    }

    pub fn decoder(mut self, decoder: fn(&Genotype<TGene>) -> T) -> Self {
        self.decoder = Some(decoder);
        self
    }
}