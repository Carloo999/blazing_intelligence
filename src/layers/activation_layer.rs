use nalgebra::DVector;
use crate::layers::activation_function::ActivationFunction;
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic, ConvertToLayerEnum};
use crate::models::model_management::layer_enum::LayerEnum;

#[derive(Clone)]
pub(crate) struct ActivationLayer {
    pub(crate) activation_function: ActivationFunction,
    pub(crate) last_input: Option<DVector<f64>>,
}

impl ForwardPropagation for ActivationLayer {
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64> {
        self.last_input = Some(input.clone());
        input.map(self.activation_function.function)
   }
}

impl BackwardPropagationStochastic for ActivationLayer {
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,_learning_rate: &f64) -> DVector<f64>{
        match &self.last_input {
            None => { panic!("cannot perform backpropagation without running the forward pass first") }
            Some(last_input) => {
                let derivative_vals = last_input.map(self.activation_function.derivative);
                output_grad.component_mul(&derivative_vals)
            }
        }
    }
}

impl ConvertToLayerEnum for ActivationLayer {
    fn convert_to_enum(&self) -> LayerEnum {
        LayerEnum::ActivationLayer(self.activation_function.function_name.clone())
    }
}

impl Layer for ActivationLayer {}