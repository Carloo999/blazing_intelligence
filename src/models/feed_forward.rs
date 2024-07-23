use nalgebra::DVector;
use crate::data::dataset::Dataset;
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::model::{Model};
use crate::layers::layer::Layer;
use crate::layers::layer_enum::LayerEnum;
use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::model_enum::ModelEnum;
use crate::models::model_manager::ConvertToModelEnum;

pub(crate) struct FeedForward {
    pub(crate) layers: Vec<Box<dyn Layer>>,
    pub(crate) training_context: TrainingContext,
    pub(crate) learning_rate_adjuster: Box<dyn LearningRateAdjuster>,
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

impl ConvertToModelEnum for FeedForward {
    fn convert_to_enum(&self) -> ModelEnum {
        let mut layers: Vec<LayerEnum> = vec![];
        for layer in self.layers.iter(){
            layers.push(layer.convert_to_enum())
        }
        ModelEnum{
            layers,
            training_context: self.training_context.clone(),
            learning_rate_adjuster: self.learning_rate_adjuster.convert_to_adjuster_enum(),
        }
    }
}