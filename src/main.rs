use simulated_annealing::init_functions::{get_towns_from_database, create_distance_matrix};
use simulated_annealing::town::Town;
use simulated_annealing::matrix::DistanceMatrix;
fn main() {
    let base: String = "base1.txt".to_string();
    let towns: Vec<Town> = get_towns_from_database(base);
    let _matrix: DistanceMatrix = create_distance_matrix(&towns);
        
}
