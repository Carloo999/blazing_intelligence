use nalgebra::DVector;
use crate::layers::activation_function::ActivationFunction;
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

struct ActivationLayer {
    activation_function: ActivationFunction,
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

impl Layer for ActivationLayer {}