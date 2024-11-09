use std::sync::Arc;

use crate::engines::genome::genotype::Genotype;
use crate::engines::genome::genes::gene::Gene;
use crate::engines::score::Score;


pub trait Problem<'a, G, A, T> 
where 
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a
{
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score;
    fn encode(&self) -> Genotype<G, A>;
    fn decode(&self, genotype: &Genotype<G, A>) -> T;
}

pub struct  DefaultProblem<'a, G, A, T> 
where 
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a
{
    pub fitness_fn: Option<Arc<dyn Fn(&T) -> Score>>,
    pub encoder_fn: Option<Arc<dyn Fn() -> Genotype<G, A>>>,
    pub decoder_fn: Option<Arc<dyn Fn(&Genotype<G, A>) -> T>>,
    _phantom: std::marker::PhantomData<&'a G>,
}

impl<'a, G, A, T> DefaultProblem<'a, G, A, T> 
where 
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a
{
    pub fn new() -> Self {
        DefaultProblem {
            fitness_fn: None,
            encoder_fn: None,
            decoder_fn: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn encoder(mut self, encoder: Arc<dyn Fn() -> Genotype<G, A>>) -> Self {
        self.encoder_fn = Some(encoder);
        self
    }

    pub fn decoder(mut self, decoder: Arc<dyn Fn(&Genotype<G, A>) -> T>) -> Self {
        self.decoder_fn = Some(decoder);
        self
    }

    pub fn fitness_fn(mut self, fitness_fn: Arc<dyn Fn(&T) -> Score>) -> Self {
        self.fitness_fn = Some(fitness_fn);
        self
    }
}


impl<'a, G, A, T> Problem<'a, G, A, T> for DefaultProblem<'a, G, A, T> 
where 
    G: Gene<G, A> + 'a,
    A: 'a,
    T: 'a
{
    fn evaluate(&self, genotype: &Genotype<G, A>) -> Score {
        let decoded = self.decode(genotype);
        match &self.fitness_fn {
            Some(fitness_fn) => fitness_fn(&decoded),
            None => panic!("Fitness function not set"),
        }
    }

    fn encode(&self) -> Genotype<G, A> {
        match &self.encoder_fn {
            Some(encoder) => encoder(),
            None => panic!("Encoder not set"),
        }
    }

    fn decode(&self, genotype: &Genotype<G, A>) -> T {
        match &self.decoder_fn {
            Some(decoder) => decoder(genotype),
            None => panic!("Decoder not set"),
        }
    }
}