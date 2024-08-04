use nalgebra::DVector;
use crate::layers::activation_function::ActivationFunction;
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic, ConvertToLayerEnum};
use crate::models::model_management::layer_enum::LayerEnum;

#[derive(Clone)]
pub struct ActivationLayer {
    pub(crate) activation_function: ActivationFunction,
}

impl ForwardPropagation for ActivationLayer {
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64> {
        input.map(self.activation_function.function)
   }
}

impl BackwardPropagationStochastic for ActivationLayer {
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,_learning_rate: &f64) -> DVector<f64>{
        output_grad.map(self.activation_function.derivative)
    }
}

impl ConvertToLayerEnum for ActivationLayer {
    fn convert_to_enum(&self) -> LayerEnum {
        LayerEnum::ActivationLayer(self.activation_function.function_name.clone())
    }
}

impl Layer for ActivationLayer {}

impl ActivationLayer {
    pub fn new(activation_function: ActivationFunction) -> ActivationLayer {
        ActivationLayer {
            activation_function
        }
    }
}