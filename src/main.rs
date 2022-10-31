use simulated_annealing::sim_annealing::{generate_initial_permutation, generate_neighbor};
use simulated_annealing::town::Town;
use simulated_annealing::matrix::DistanceMatrix;
fn main() {
    let base: String = "base1.txt".to_string();
    let mut towns: Vec<Town> = generate_initial_permutation(base);
    let towns = generate_neighbor(&mut towns, 2);
    let _matrix: DistanceMatrix = DistanceMatrix::create_distance_matrix(&towns);
}
