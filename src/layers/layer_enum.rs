use crate::layers::{dense_layer::DenseLayer, activation_layer::ActivationLayer};
use crate::layers::activation_function::{ActivationFunction};
use crate::layers::function_name::FunctionName;
use crate::layers::layer::Layer;
use crate::layers::layer_enum::LayerEnum::{Dense_Layer, Activation_Layer};
pub enum LayerEnum {
    Dense_Layer(DenseLayer),
    Activation_Layer(FunctionName),
}

impl LayerEnum {
    pub fn convert_to_layer(&self,custom_activation_function: Option<ActivationFunction>) -> Box<dyn Layer>{
        return match self {
            LayerEnum::Dense_Layer(x) => Box::new(x.clone()),
            LayerEnum::Activation_Layer(x) => {
                Box::new(ActivationLayer{activation_function: x.convert_to_activation_function(custom_activation_function)})
            },
        }
    }
}