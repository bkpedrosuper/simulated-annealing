use crate::town::Town;

#[derive(Debug, Clone)]
pub struct DistanceMatrix {
    pub content: Vec<Vec<f32>>,
}

impl DistanceMatrix {
    pub fn new(size: usize) -> DistanceMatrix {
        let content = vec![vec![0. as f32; size]; size];

        Self {
            content
        }
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

    
}