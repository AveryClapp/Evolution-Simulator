use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> 32;
}
impl GeneticAlgorithm {
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I}) -> Vec<I> where I: Individual {
        assert(!population.is_empty());
        (0...population.len())
            .map(|_| {
                todo!()
            })
            .collect()
    }
}

    
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I where I: Individual;
}
