use nalgebra::DVector;
use crate::layers::activation_function::ActivationFunction;
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic, ConvertToLayerEnum};
use crate::models::model_management::layer_enum::LayerEnum;

/// ActivationLayer struct which contains the activation function and the last input
#[derive(Clone)]
pub struct ActivationLayer {
    pub(crate) activation_function: ActivationFunction,
    pub(crate) last_input: Option<DVector<f64>>,
}

impl ForwardPropagation for ActivationLayer {
    /// Forward propagates the input through the activation function f(x)
    ///<br> output = f(input)
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64> {
        self.last_input = Some(input.clone());
        input.map(self.activation_function.function)
   }
}

impl BackwardPropagationStochastic for ActivationLayer {
    /// Backward propagates the output gradient through the activation function f(x)
    /// and returns the gradient of the input
    /// <br> input_grad = f'(last_input) * output_grad
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
    /// Converts the ActivationLayer to a LayerEnum which is the savable type for layers
    fn convert_to_enum(&self) -> LayerEnum {
        LayerEnum::ActivationLayer(self.activation_function.function_name.clone())
    }
}

impl Layer for ActivationLayer {}

impl ActivationLayer {
    /// Constructs a new ActivationLayer with the given activation function
    pub fn new(activation_function: ActivationFunction) -> ActivationLayer {
        ActivationLayer {
            activation_function,
            last_input: None,
        }
    }
}