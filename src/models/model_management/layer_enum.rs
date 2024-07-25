use crate::layers::{dense_layer::DenseLayer, activation_layer::ActivationLayer};
use crate::layers::activation_function::{ActivationFunction};
use crate::layers::function_name::FunctionName;
use crate::layers::layer::Layer;

use savefile::prelude::*;
use crate::models::model_management::dense_layer_savable::DenseLayerSavable;
#[derive(Savefile)]
pub enum LayerEnum {
    DenseLayer(DenseLayerSavable),
    ActivationLayer(FunctionName),
}

impl LayerEnum {
    pub fn convert_to_layer(&self,custom_activation_function: Option<ActivationFunction>) -> Box<dyn Layer>{
        return match self {
            LayerEnum::DenseLayer(x) => Box::new(x.to_dense_layer()),
            LayerEnum::ActivationLayer(x) => {
                Box::new(ActivationLayer{activation_function: x.convert_to_activation_function(custom_activation_function)})
            },
        }
    }
}