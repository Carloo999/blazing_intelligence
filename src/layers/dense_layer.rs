use nalgebra::{DMatrix, DVector};
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

struct DenseLayer {
    weights: DMatrix<f64>,
    biases: DVector<f64>,
    last_input: Option<DVector<f64>>,
}


impl ForwardPropagation for DenseLayer {
    fn forwards_propagate(&mut self) -> DVector<f64> {
        todo!()
    }
}

impl BackwardPropagationStochastic for DenseLayer {
    fn backwards_propagate(&mut self, learning_rate: &f64) -> DVector<f64> {
        todo!()
    }
}

impl Layer for DenseLayer {}