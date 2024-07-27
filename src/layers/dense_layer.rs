use nalgebra::{DMatrix, DVector};
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

pub struct DenseLayer {
    weights: DMatrix<f64>,
    biases: DVector<f64>,
    last_input: Option<DVector<f64>>,
}


impl ForwardPropagation for DenseLayer {
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64> {
        self.last_input = Some(input.clone());
        &self.weights * input + &self.biases
    }
}

impl BackwardPropagationStochastic for DenseLayer {
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,learning_rate: &f64) -> DVector<f64> {
        let input_grad: DVector<f64> = &self.weights.transpose() * output_grad;
        match &self.last_input {
            Some(last_input) => {
                self.weights -= (output_grad * last_input.transpose()).map(|x| x*learning_rate);
                self.biases -= output_grad.map(|x| x*learning_rate);
            },
            None => panic!("cannot perform backpropagation without running the forward pass first")
        }
        input_grad
    }
}

impl Layer for DenseLayer {}

impl DenseLayer {
    pub fn new(input_amount: usize, neuron_amount: usize) -> DenseLayer {
        DenseLayer {
            weights: DMatrix::new_random(neuron_amount, input_amount),
            biases: DVector::new_random(neuron_amount),
            last_input: None,
        }
    }
}