use std::fs;

use crate::town::Town;
// use crate::town::Town;

pub fn get_towns_from_database(base: String) ->  Vec<Town> {
    let base_path: String = format!("base/{base_path}", base_path=base);

    let mut results: Vec<Town> = Vec::new();

    let mut id: usize = 0;
    
    let content = fs::read_to_string(base_path).expect("Could not read database file");
    for line in content.split("\n") {
        if line.is_empty() || line.starts_with("#") || line.starts_with("0") { continue }

        let values: Vec<&str> = line.split_whitespace().collect();
        let (x, y) = (values[1].parse::<i32>().unwrap(), values[2].parse::<i32>().unwrap());

        results.push(Town{x, y, id});
        id+=1;
    }
    results
}