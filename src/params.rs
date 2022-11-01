
pub struct Params {
    pub alpha: f32,
    pub max_iter: usize,
    pub temp0: f32,
    pub n_swaps: usize,
    pub temp_final: f32,
    pub k: f32,
}

impl Params {
    pub fn new(
        alpha: f32,
        max_iter: usize,
        temp0: f32,
        n_swaps: usize,
        temp_final: f32,
        k: f32,
    ) -> Self {

        Self {
            alpha,
            max_iter,
            temp0,
            n_swaps,
            temp_final,
            k,
        }
    }

    pub fn default() -> Self {
        
        Params { alpha: (0.98), max_iter: (100000), temp0: (1.), n_swaps: (1), temp_final: (0.9), k: (1.)}
    }
}