use crate::town::{Town, generate_neighbor};
use crate::matrix::DistanceMatrix;
use crate::params::Params;
use rand::{Rng};
use std::f32::consts::E;

fn energy_probability_function(delta: f32, k: f32, t: f32) -> f32{
    E.powf(-delta / (k * t))
}

fn cooling_schedule(t0: &f32, tn: &f32, iter: &usize, n: &usize) -> f32 {
    let exp = (*iter as f32) / (*n as f32);
    t0 * ((tn/t0).powf(exp))
}

pub fn tsp_sa_algorithm(mut towns: Vec<Town>, matrix: &DistanceMatrix, params: Params) -> Vec<Town> {
    let mut temp = params.temp0;
    let mut best_towns: Vec<Town> = towns.to_vec();
    let mut best_distance: f32 = matrix.tsp_checker(&towns);
    
    let mut iterations: usize = 0;
    
    while temp > params.temp_final {
        iterations += 1;

        for _ in 0..params.max_iter {

            let neighbor = generate_neighbor(&mut towns, params.n_swaps);
            let total_distance = matrix.tsp_checker(&neighbor);
            let delta = total_distance - matrix.tsp_checker(&towns);

            if delta < 0. {
                towns = neighbor;
    
                if total_distance < best_distance {
                    best_distance = total_distance;
                    best_towns = towns.to_vec();
                }
            }
            
            else {
                let mut rng = rand::thread_rng();
                let x: f32 = rng.gen();
                
                if x < energy_probability_function(delta, params.k, temp) {
                    towns = neighbor;
                }
            }

            
        }
        
        // temp = temp * params.alpha;
        temp = cooling_schedule(&params.temp0, &params.temp_final, &iterations,  &params.max_iter);
        
        println!("Temperatura: {}", temp);
        println!("Distância: {}", matrix.tsp_checker(&towns));
    }
    
    best_towns
}