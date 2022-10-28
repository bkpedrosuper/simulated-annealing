
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

    
}