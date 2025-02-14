use nalgebra::{DVector, VecStorage};
use crate::models::model_management::layer_enum::LayerEnum;

/// Layer trait to be implemented by every type of layer
/// <br> Layers must implement ForwardPropagation, BackwardPropagationStochastic, and ConvertToLayerEnum
pub trait Layer: ForwardPropagation + BackwardPropagationStochastic + ConvertToLayerEnum{}

/// ForwardPropagation trait to be implemented by layers that can forward propagate
pub trait ForwardPropagation{
    fn forwards_propagate(&mut self, input: &DVector<f64>) -> DVector<f64>;
}

/// BackwardPropagationStochastic trait to be implemented by layers that can backward propagate
pub trait BackwardPropagationStochastic{
    fn backwards_propagate(&mut self, output_grad: &DVector<f64>,learning_rate: &f64) -> DVector<f64>;
}

/// ConvertToLayerEnum trait to be implemented by layers that can be converted to LayerEnum to be saved to a file
pub trait ConvertToLayerEnum{
    fn convert_to_enum(&self) -> LayerEnum;
}
