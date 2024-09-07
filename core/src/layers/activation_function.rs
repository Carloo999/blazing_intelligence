use crate::layers::function_name::FunctionName;
//to add new ActivationFunction constructors,
//add the name of the new function to the FunctionName enum
//and extend the match block in the "convert_to_activation_function()" function.
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

    pub fn sigmoid() -> ActivationFunction {
        ActivationFunction{
            function_name: FunctionName::Sigmoid,
            function: |x| 1. / (1. + f64::exp(-1. * x)),
            derivative: |x| {
                let y = 1. / (1. + f64::exp(-1. * x));
                y * (1. - y)
            },
        }
    }

    pub fn relu() -> ActivationFunction {
        ActivationFunction{
            function_name: FunctionName::RELU,
            function: |x| x.max(0.),
            derivative: |x| if x > 0. {1.} else {0.},
        }
    }

    pub fn gelu() -> ActivationFunction {
        ActivationFunction{
            function_name: FunctionName::GELU,
            function: |x| x * sigmoid(&x),
            derivative: |x| x * sigmoid(&x) + sigmoid(&x),
        }
    }

}

impl Default for ActivationFunction {
    fn default() -> Self {
        ActivationFunction::tanh()
    }
}



fn sigmoid(x: &f64) -> f64{
    1. / (1. + f64::exp(-1. * x))
}

fn sigmoid_derivative(x: &f64) -> f64{
    let y = 1. / (1. + f64::exp(-1. * x));
    y * (1. - y)
}