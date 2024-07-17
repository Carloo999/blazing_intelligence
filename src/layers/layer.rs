use nalgebra::{DVector, VecStorage};

pub(crate) trait Layer: ForwardPropagation + BackwardPropagationStochastic{}

pub(crate) trait ForwardPropagation{
    fn forwards_propagate(&mut self) -> DVector<f64>;
}

pub(crate) trait BackwardPropagationStochastic{
    fn backwards_propagate(&mut self, learning_rate: &f64) -> DVector<f64>;
}
