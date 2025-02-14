use std::ops::Deref;
use crate::layers::layer::Layer;
use crate::models::model_management::layer_enum::LayerEnum;
use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::feed_forward::FeedForward;
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
use crate::layers::activation_function::ActivationFunction;
use savefile::prelude::*;
/// ModelEnum struct which contains the layers, training context, and learning rate adjuster
/// <br> This struct is used to save and load entire models
#[derive(Savefile)]
pub struct ModelEnum{
    pub(crate) layers: Vec<LayerEnum>,
    pub training_context: TrainingContext,
    pub(crate) learning_rate_adjuster: LearningRateAdjusterEnum,
}

impl ModelEnum{
    /// Converts the ModelEnum to a FeedForward model on which training and prompting can be performed
    pub fn convert_to_feed_forward_model(&self, custom_activation_function: Option<ActivationFunction>) -> Box<FeedForward> {
        let mut layers: Vec<Box<dyn Layer>> = vec![];
        for layer in self.layers.iter(){
            layers.push(layer.convert_to_layer(custom_activation_function.clone()))
        }
        Box::new(FeedForward{
            layers,
            training_context: self.training_context.clone(),
            learning_rate_adjuster: self.learning_rate_adjuster.convert_to_learning_rate_adjuster(),
        })
    }

    /// Saves the ModelEnum to a file at the given path
    pub fn save(&self,path: &str) -> Result<(),SavefileError>{
        save_file(path, 0, self)
    }

    /// Loads a ModelEnum from a file at the given path
    pub fn load(path: &str) -> Result<ModelEnum, SavefileError>{
        load_file(path, 0)
    }
}