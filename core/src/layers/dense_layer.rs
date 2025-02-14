use nalgebra::{DMatrix, DVector};
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic, ConvertToLayerEnum};
use crate::models::model_management::layer_enum::LayerEnum;
use crate::models::model_management::dense_layer_savable::DenseLayerSavable;
use crate::utilities::type_conversion::ToSavable;
/// DenseLayer struct which contains the weights, biases, and the last input
#[derive(Clone)]
pub struct DenseLayer {
    pub weights: DMatrix<f64>,
    pub biases: DVector<f64>,
    pub last_input: Option<DVector<f64>>,
}

impl ForwardPropagation for DenseLayer {
    /// Forward propagates the input through the layer
    /// <br> output = weights * input + biases
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64> {
        self.last_input = Some(input.clone());
        &self.weights * input + &self.biases
    }
}

impl BackwardPropagationStochastic for DenseLayer {
    /// Backward propagates the output gradient through the layer, updating the weights and biases, returning the gradient of the input
    /// <br> input_grad = weights^T * output_grad
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

impl ConvertToLayerEnum for DenseLayer {
    /// Converts the DenseLayer to a LayerEnum which is the savable type for layers
    fn convert_to_enum(&self) -> LayerEnum {
        LayerEnum::DenseLayer(self.to_savable())
    }
}

impl DenseLayer{
    /// Converts the DenseLayer to a DenseLayerSavable struct
    fn to_savable(&self) -> DenseLayerSavable{
        let last_input: Option<Vec<f64>>;

        let weights:Vec<Vec<f64>>= self.weights.to_savable();
        let biases: Vec<f64>= self.biases.to_savable();
        match &self.last_input {
            Some(x) => {
                last_input = Some(x.to_savable())
            }
            None => {last_input = None}
        }
        DenseLayerSavable{
            weights,
            biases,
            last_input,
        }
    }
}

impl Layer for DenseLayer {}

impl DenseLayer {
    /// Constructs a new DenseLayer with the given input and neuron amount
    pub fn new(input_amount: usize, neuron_amount: usize) -> DenseLayer {
        DenseLayer {
            weights: DMatrix::new_random(neuron_amount, input_amount),
            biases: DVector::new_random(neuron_amount),
            last_input: None,
        }
    }
}