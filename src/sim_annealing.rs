use crate::town::Town;
use crate::init_functions::get_towns_from_database;
use rand::{thread_rng, seq::SliceRandom, distributions::Uniform, prelude::Distribution};

pub fn generate_initial_permutation(base: String) -> Vec<Town> {
    let mut towns: Vec<Town> = get_towns_from_database(base);
    println!("Before shuffle \n{:?}", towns);
    towns.shuffle(&mut thread_rng());
    println!("After shuffle \n{:?}", towns);

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
    println!("AFTER MOVE \n {:?}", towns);
    towns.to_vec()
}
// fn generate_permutation(towns: Vec)

// pub fn sa_algorithm(towns: Vec<Town>) {
//     println!("FEZ O ALGORITMO CONFIA");
// }