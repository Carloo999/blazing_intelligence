use crate::layers::function_name::FunctionName;

#[derive(Clone)]
pub struct ActivationFunction{
    pub function_name: FunctionName,
    pub function: fn(f64) -> f64,
    pub derivative: fn(f64) -> f64,
}

impl ActivationFunction {
    pub fn tanh() -> ActivationFunction{
        ActivationFunction{
            function_name: FunctionName::Tanh,
            function: |x| x.tanh(),
            derivative: |x| 1. - x.tanh().powi(2),
        }
    }
}

impl Default for ActivationFunction {
    fn default() -> Self {
        ActivationFunction::tanh()
    }
}

