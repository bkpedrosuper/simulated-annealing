use std::fs;

use crate::town::Town;
use crate::matrix::DistanceMatrix;
// use crate::town::Town;

pub fn get_towns_from_database(base: String) ->  Vec<Town> {
    let base_path: String = format!("base/{base_path}", base_path=base);

    let mut results: Vec<Town> = Vec::new();
    let a = Town::new(0 as i32, 0 as i32);
    results.push(a);
    
    let content = fs::read_to_string(base_path).expect("Could not read database file");
    for line in content.split("\n") {
        if line.is_empty() || line.starts_with("#") || line.starts_with("0") { continue }

        let values: Vec<&str> = line.split_whitespace().collect();
        let (x, y) = (values[1].parse::<i32>().unwrap(), values[2].parse::<i32>().unwrap());

        results.push(Town{x, y});
    }
    results
}

pub fn create_distance_matrix(towns: &Vec<Town>) ->  DistanceMatrix {
    let inf: f32 = 10000007.;
    let size = towns.len() as usize;
    let mut matrix: DistanceMatrix = DistanceMatrix::new(size);

    for town in 0 .. towns.len() {
        for k in 0 .. towns.len() {
            if k == 0 || town == 0 {
                matrix.content[town][k] = inf;
                continue;
            }
            
            let dist = Town::dist(&towns[town], &towns[k]);
            
            matrix.content[town][k] = dist;
        }
    }

    matrix
}