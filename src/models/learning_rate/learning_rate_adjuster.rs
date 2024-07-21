use crate::models::learning_rate::training_context::TrainingContext;

pub trait LearningRateAdjuster {
    fn adjust(&mut self, context: TrainingContext);
    fn get_learning_rate(&self) -> f64;
}

