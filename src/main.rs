use simulated_annealing::io_functions::create_file;
use simulated_annealing::sim_annealing::{tsp_sa_algorithm};
use simulated_annealing::town::{Town, generate_initial_permutation};
use simulated_annealing::matrix::DistanceMatrix;
use simulated_annealing::params::Params;
fn main() {

    let total_runs = 10;
    // Algoritmo
    let params = Params { alpha: (0.98),
        max_iter: (10e2 as usize),
        temp0: (1.),
        n_swaps: (1),
        temp_final: (0.1),
        k: (1.),
        base: "base51.txt".to_string()
    };

    create_file(&params.base).expect("Could not create file");
    
    for _ in 0..total_runs {
        let towns: Vec<Town> = generate_initial_permutation(&params.base);
        let matrix: DistanceMatrix = DistanceMatrix::create_distance_matrix(&towns);
        let results = tsp_sa_algorithm(towns, &matrix, &params);

        println!("{}", matrix.tsp_checker(&results));    
    }
    // let results = tsp_sa_algorithm(towns, &matrix, params);

    // println!("Results: {:?}", results);
    // println!("Distance: {}", matrix.tsp_checker(&results));
}
