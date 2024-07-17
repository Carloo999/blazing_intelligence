pub struct ActivationFunction{
    pub function: fn(f64) -> f64,
    pub derivative: fn(f64) -> f64,
}

impl ActivationFunction {
    pub fn tanh() -> ActivationFunction{
        ActivationFunction{
            function: |x| x.tanh(),
            derivative: |x| 1. - x.tanh().powi(2),
        }
    }
}
