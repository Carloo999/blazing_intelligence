#[derive(Savefile,Clone)]
pub struct TrainingContext {
    pub mse_evolution: Vec<f64>,
    pub epoch_count: usize,
    pub prev_learning_rate: f64,
}