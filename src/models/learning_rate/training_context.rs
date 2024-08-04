#[derive(Savefile,Clone)]
pub struct TrainingContext {
    pub mse_evolution: Vec<f64>,
    pub epoch_count: usize,
    pub prev_learning_rate: f64,
}

impl TrainingContext {
    pub fn new_empty() -> TrainingContext {
        TrainingContext {
            mse_evolution: vec![],
            epoch_count: 0,
            prev_learning_rate: 0.0,
        }
    }
}