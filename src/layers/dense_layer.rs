use nalgebra::{DMatrix, DVector};
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

struct DenseLayer {
    weights: DMatrix<f64>,
    biases: DVector<f64>,
    last_input: Option<DVector<f64>>,
}


impl ForwardPropagation for DenseLayer {}

impl BackwardPropagationStochastic for DenseLayer {}

impl Layer for DenseLayer {}