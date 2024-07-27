use nalgebra::{DMatrix, DVector};
use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic, ConvertToLayerEnum};
use crate::models::model_management::layer_enum::LayerEnum;
use crate::models::model_management::dense_layer_savable::DenseLayerSavable;
use crate::nalgebra_utilities::type_conversion::ToSavable;

#[derive(Clone)]
pub(crate) struct DenseLayer {
    pub(crate) weights: DMatrix<f64>,
    pub(crate) biases: DVector<f64>,
    pub(crate) last_input: Option<DVector<f64>>,
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

impl ConvertToLayerEnum for DenseLayer {
    fn convert_to_enum(&self) -> LayerEnum {
        LayerEnum::DenseLayer(self.to_savable())
    }
}

impl DenseLayer{
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
    pub fn new(input_amount: usize, neuron_amount: usize) -> DenseLayer {
        DenseLayer {
            weights: DMatrix::new_random(neuron_amount, input_amount),
            biases: DVector::new_random(neuron_amount),
            last_input: None,
        }
    }
}