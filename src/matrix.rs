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
        let size = towns.len() as usize;
        let mut matrix: DistanceMatrix = DistanceMatrix::new(size);

        for town1 in towns {
            for town2 in towns {
                let dist = Town::dist(&town1, &town2);
                
                matrix.content[town1.id][town2.id] = dist;
            }
        }
    
        matrix
    }

    pub fn tsp_checker(&self, towns: &Vec<Town>) -> f32 {
        let mut dist: f32 = 0.;
        for i in 0..towns.len() - 1 {
            assert!(i <= towns.len() - 1);
            dist += self.content[towns[i].id][towns[i+1].id];
        }

        dist
    }

    
}