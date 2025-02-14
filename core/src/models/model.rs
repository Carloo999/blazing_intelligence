use nalgebra::DVector;
use crate::data::dataset::Dataset;
use crate::models::model_management::model_enum::ModelEnum;
use crate::models::model_management::model_manager::ConvertToModelEnum;

/// Model trait to be implemented by every type of model
/// <br> Models must implement prompt, train_gradient_desc, train_stochastic_gradient_desc, and test_network
pub trait Model: ConvertToModelEnum{
    fn prompt(&mut self, input: DVector<f64>) -> DVector<f64>;
    fn train_gradient_desc(&mut self, epochs: usize, batch_size: usize, training_dataset: Dataset);
    fn train_stochastic_gradient_desc(&mut self, epochs: usize, dataset: Dataset);
    fn test_network(&mut self, test_dataset: Dataset);
}
