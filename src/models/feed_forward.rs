use nalgebra::{DVector, dvector};
use crate::data::dataset::Dataset;
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::model::Model;
use crate::layers::layer::Layer;
use crate::models::learning_rate::training_context::TrainingContext;

pub struct FeedForward {
    layers: Vec<Box<dyn Layer>>,
    training_context: Option<TrainingContext>,
    learning_rate_adjuster: Box<dyn LearningRateAdjuster>,
}

impl Model for FeedForward {
    fn prompt(&mut self, mut input: DVector<f64>) -> DVector<f64> {
        for mut layer in self.layers.iter_mut() {
            input = layer.forwards_propagate(&input);
        }
        input
    }

    fn train(&mut self, dataset: Dataset) {
        todo!()
    }
}

impl FeedForward {
    pub fn new(layer_structure: Vec<Box<dyn Layer>>, learning_rate_adjuster: Box<dyn LearningRateAdjuster>) -> Box<FeedForward> {
        let network = FeedForward {
            layers: layer_structure,
            training_context: None,
            learning_rate_adjuster,
        };
        Box::new(network)
    }
}
