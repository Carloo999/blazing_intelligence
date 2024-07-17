use nalgebra::DVector;
use crate::layers::activation_function::ActivationFunction;
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

struct ActivationLayer {
    activation_function: ActivationFunction,
}

impl ForwardPropagation for ActivationLayer {
    fn forwards_propagate(&mut self) -> DVector<f64> {
        todo!()
    }
}

impl BackwardPropagationStochastic for ActivationLayer {
    fn backwards_propagate(&mut self, learning_rate: &f64) -> DVector<f64>{
        todo!()
    }
}

impl Layer for ActivationLayer {}