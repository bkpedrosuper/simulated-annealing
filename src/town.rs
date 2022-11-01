use crate::init_functions::get_towns_from_database;
use rand::{thread_rng, seq::SliceRandom, distributions::Uniform, prelude::Distribution};

#[derive(Debug, Clone)]
pub struct Town {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

impl Town {
    pub fn new(x: i32, y: i32, id: usize) -> Self {
        Self {
            x,
            y,
            id,
        }
    }

    pub fn dist(el1: &self::Town, el2: &self::Town) -> f32 {
        // Euclidian Distance
        (((el1.x - el2.x) as f32).powi(2) + ((el1.y - el2.y) as f32).powi(2)).sqrt()
    }
}

pub fn generate_initial_permutation(base: String) -> Vec<Town> {
    let mut towns: Vec<Town> = get_towns_from_database(base);
    towns.shuffle(&mut thread_rng());

    towns.to_vec()
}

pub fn generate_neighbor(towns: &mut Vec<Town>, n_swaps: usize) -> Vec<Town> {
    let mut rng = rand::thread_rng();
    let towns_range = Uniform::from(0..towns.len());

    for _ in 0..n_swaps {
        let pick1 = towns_range.sample(&mut rng);
        let pick2 = towns_range.sample(&mut rng);
        println!("Mudou o {} e o {}", pick1, pick2);
        towns.swap(pick1, pick2)
    }
    towns.to_vec()
}