use nalgebra::{DVector, dvector};

pub struct MathUtils {

}

impl MathUtils {
    pub fn calculate_mse(pred: &DVector<f64>, correct: &DVector<f64>) -> f64 {
        assert_eq!(correct.len(), pred.len());
        let total: f64 = correct.iter().zip(pred.iter()).map(|(c, p)| (c - p).powi(2)).sum();
        total / correct.len() as f64
    }

    pub fn calculate_mse_prime(pred: &DVector<f64>, correct: &DVector<f64>) -> DVector<f64> {
        assert_eq!(correct.len(), pred.len());
        let derivatives = correct.iter()
            .zip(pred.iter())
            .map(|(c, p)| {2.0 * (p - c) / correct.len() as f64})
            .collect();
        DVector::from_vec(derivatives)
    }
}