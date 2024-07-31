use nalgebra::DVector;
use crate::data::dataset::Dataset;
use crate::models::model_management::model_enum::ModelEnum;

pub trait Model{
    fn prompt(&mut self, input: DVector<f64>) -> DVector<f64>;
    fn train_gradient_desc(&mut self, epochs: usize, batch_size: usize, dataset: Dataset);
    fn train_stochastic_gradient_desc(&mut self, epochs: usize, dataset: Dataset);
}
