use log::info;
use crate::layers::activation_function::ActivationFunction;

#[derive(Clone)]
#[derive(Default)]
pub enum FunctionName {
    #[default]
    Tanh,
    Custom,
}

impl FunctionName {
    pub fn convert_to_activation_function(&self,custom_function: Option<ActivationFunction>) -> ActivationFunction{
        match (&custom_function, self) {
            (Some(_),_) => custom_function.unwrap(),
            (None, FunctionName::Custom) => {info!("Network was created with custom activation function but none was supplied, defaulting to tanh.");
                                            ActivationFunction::default()},
            (None, FunctionName::Tanh) => {ActivationFunction::tanh()}
        }
    }
}