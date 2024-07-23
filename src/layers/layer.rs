use nalgebra::{DVector, VecStorage};
use crate::layers::layer_enum::LayerEnum;

pub(crate) trait Layer: ForwardPropagation + BackwardPropagationStochastic + ConvertToLayerEnum{}

pub(crate) trait ForwardPropagation{
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64>;
}

pub(crate) trait BackwardPropagationStochastic{
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,learning_rate: &f64) -> DVector<f64>;
}

pub trait ConvertToLayerEnum{
    fn convert_to_enum(&self) -> LayerEnum;
}
