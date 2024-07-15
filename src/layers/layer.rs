pub(crate) trait Layer: ForwardPropagation + BackwardPropagationStochastic{}

pub(crate) trait ForwardPropagation{
}

pub(crate) trait BackwardPropagationStochastic{
    fn backwards_propagate(&mut self, learning_rate: &f64){
    }
}
