use nalgebra::{DVector, VecStorage};
use crate::models::model_management::layer_enum::LayerEnum;

pub trait Layer: ForwardPropagation + BackwardPropagationStochastic + ConvertToLayerEnum{}

pub trait ForwardPropagation{
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64>;
}

pub trait BackwardPropagationStochastic{
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,learning_rate: &f64) -> DVector<f64>;
}

pub trait ConvertToLayerEnum{
    fn convert_to_enum(&self) -> LayerEnum;
}
