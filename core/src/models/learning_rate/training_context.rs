/// TrainingContext is a struct that holds the training context of the neural network.
/// It contains the mean squared error evolution (Vector of previous MSEs), the epoch count and the previous learning rate.
#[derive(Savefile,Clone)]
pub struct TrainingContext {
    pub mse_evolution: Vec<f64>,
    pub epoch_count: usize,
    pub prev_learning_rate: f64,
}

impl TrainingContext {
    /// Constructs a new TrainingContext with empty mse_evolution, epoch_count = 0 and prev_learning_rate = 0.0
    pub fn new_empty() -> TrainingContext {
        TrainingContext {
            mse_evolution: vec![],
            epoch_count: 0,
            prev_learning_rate: 0.0,
        }
    }
}