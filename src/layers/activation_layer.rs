use crate::layers::layer::{Layer, ForwardPropagation, BackwardPropagationStochastic};

struct ActivationLayer {

}

impl ForwardPropagation for ActivationLayer {}

impl BackwardPropagationStochastic for ActivationLayer {}

impl Layer for ActivationLayer {

}