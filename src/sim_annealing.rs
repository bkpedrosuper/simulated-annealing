use crate::town::{Town, generate_neighbor};
use crate::matrix::DistanceMatrix;
use crate::params::Params;
use rand::{Rng};
use std::f32::consts::E;

fn energy_probability_function(delta: f32, k: f32, t: f32) -> f32{
    E.powf(-delta / (k * t))
}

pub fn tsp_sa_algorithm(mut towns: Vec<Town>, matrix: &DistanceMatrix, params: Params) -> Vec<Town> {
    let mut temp = params.temp0;

    while temp > params.temp_final {

        for _ in 0..params.max_iter {

            let neighbor = generate_neighbor(&mut towns, params.n_swaps);
            let delta = matrix.tsp_checker(&neighbor) - matrix.tsp_checker(&towns);
    
            if delta < 0. {
                towns = neighbor;
            }
            
            else {
                let mut rng = rand::thread_rng();
                let x: f32 = rng.gen();
                

                if x < energy_probability_function(delta, params.k, temp) {
                    towns = neighbor;
                }
            }

        }
        
        // print!("Best: ");
        // for town in &towns {
        //     print!("{}", town.id);
        // }
        // println!();
        temp = temp * params.alpha;
        println!("Temperatura: {}", temp);

    }
    
    towns
}