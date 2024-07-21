use nalgebra::DVector;
use crate::data::dataset::Dataset;
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
    fn prompt(&self, mut input: DVector<f64>) -> DVector<f64> {
        for mut layer in self.layers {
            input = layer.forwards_propagate(&input);
        }
        input
    }

    fn train(&mut self, dataset: Dataset) {
        todo!()
    }
}
