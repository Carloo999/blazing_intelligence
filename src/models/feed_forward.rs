use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::model::Model;
use crate::layers::layer::Layer;
use crate::models::learning_rate::training_context::TrainingContext;

struct FeedForward {
    layers: Vec<Box<dyn Layer>>,
    training_context: TrainingContext,
    learning_rate_adjuster: dyn LearningRateAdjuster,
}

impl Model for FeedForward {
    fn prompt(&self) {
        todo!()
    }

    fn train(&mut self) {
        todo!()
    }
}