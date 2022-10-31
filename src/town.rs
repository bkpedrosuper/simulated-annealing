
#[derive(Debug, Clone)]
pub struct Town {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

impl Town {
    pub fn new(x: i32, y: i32, id: usize) -> Self {
        Self {
            x,
            y,
            id,
        }
    }

    pub fn dist(el1: &self::Town, el2: &self::Town) -> f32 {
        // Euclidian Distance
        (((el1.x - el2.x) as f32).powi(2) + ((el1.y - el2.y) as f32).powi(2)).sqrt()
    }
}