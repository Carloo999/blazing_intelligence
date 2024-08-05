use std::io::Error;
use std::path::Path;
use nalgebra::{DVector, dvector};
use savefile::{load, load_file, save_file, SavefileError};
use crate::data::dataset::Dataset;
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::model::{Model};
use crate::layers::layer::Layer;
use crate::models::model_management::layer_enum::LayerEnum;
use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::model_management::model_enum::ModelEnum;
use crate::models::model_management::model_manager::{ConvertToModelEnum, ModelManager};
use crate::utilities::math_utils::MathUtils;

pub struct FeedForward {
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

    fn train_gradient_desc(&mut self, epochs: usize, batch_size: usize, dataset: Dataset) {
        todo!()
    }

    fn train_stochastic_gradient_desc(&mut self, epochs: usize, dataset: Dataset) {
        for epoch in 0..epochs {
            let mut mse_sum: f64 = 0.0;
            for (input, correct) in dataset.inputs.iter().zip(dataset.labels.iter()) {
                let output: DVector<f64> = self.prompt(input.clone());
                let mse: f64 = MathUtils::calculate_mse(&output, correct);
                let mut grad = MathUtils::calculate_mse_prime(&output, correct);
                for layer in self.layers.iter_mut().rev() {
                    grad = layer.backwards_propagate(&grad, &self.learning_rate_adjuster.get_learning_rate());
                }
                mse_sum += mse.clone();
            }
            self.training_context.mse_evolution.push(mse_sum / dataset.inputs.len() as f64);
            self.training_context.epoch_count += 1;
            self.learning_rate_adjuster.adjust(&self.training_context);
        }
    }
}

impl FeedForward {
    pub fn new(layer_structure: Vec<Box<dyn Layer>>, learning_rate_adjuster: Box<dyn LearningRateAdjuster>) -> Box<FeedForward> {
        let network = FeedForward {
            layers: layer_structure,
            training_context: TrainingContext::new_empty(),
            learning_rate_adjuster,
        };
        Box::new(network)
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

impl ModelManager for FeedForward{
    fn save(&self, filepath: &Path) -> Result<(), SavefileError> {
        let savable = self.convert_to_enum();
        save_file(filepath, 0, &savable)
    }

    fn load(filepath: &Path) -> Result<Box<dyn Model>, SavefileError> {
        let loaded : Result<ModelEnum, SavefileError> = load_file(filepath, 0);
        match loaded {
            Ok(x) => Ok(x.convert_to_feed_forward_model(None)),
            Err(x) => Err(x)
        }
    }
}