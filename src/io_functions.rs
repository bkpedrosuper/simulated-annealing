use std::fs;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use crate::town::Town;
// use crate::town::Town;

pub fn get_towns_from_database(base: &String) ->  Vec<Town> {
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

pub fn create_file(base: &String) -> std::io::Result<()> {
    File::create(format!("results/{}", base))?;
    Ok(())
}

pub fn write_results(base: &String, temp: f32, dist: f32) -> std::io::Result<()> {

    let path = format!("results/{}", base);
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", format!("{temp} {dist}", temp=temp, dist=dist)) {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    Ok(())
}

pub fn delete_file(base: &String) -> std::io::Result<()> {
    let path = format!("results/{}", base);
    fs::remove_file(path).expect("Could not remove file");
    
    Ok(())
}