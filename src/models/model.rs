use nalgebra::DVector;
use crate::data::dataset::Dataset;

pub trait Model {
    fn prompt(&mut self, input: DVector<f64>) -> DVector<f64>;
    fn train(&mut self, dataset: Dataset);
}