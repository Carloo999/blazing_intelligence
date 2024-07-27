use nalgebra::{DMatrix, DVector};
use crate::layers::dense_layer::DenseLayer;
use crate::nalgebra_utilities::type_conversion;
use crate::nalgebra_utilities::type_conversion::FromSavable;
#[derive(Savefile)]
pub struct DenseLayerSavable{
    pub(crate) weights: Vec<Vec<f64>>,
    pub(crate) biases: Vec<f64>,
    pub(crate) last_input: Option<Vec<f64>>,
}

impl DenseLayerSavable{
    pub fn to_dense_layer(&self) -> DenseLayer{
        let weights:DMatrix<f64> = <Vec<Vec<f64>> as FromSavable<f64>>::from_savable(&self.weights);
        let biases:DVector<f64> = self.biases.from_savable();
        let last_input: Option<DVector<f64>>;
        match &self.last_input {
            Some(x) => last_input = Some(x.from_savable()),
            None => last_input = None
        }
        DenseLayer{
            weights,
            biases,
            last_input,
        }
    }
}
