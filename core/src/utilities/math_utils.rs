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

    // should be optimized in the future
    pub fn index_of_max_element(d_vec: &DVector<f64>) -> usize {
        let mut index: usize = 0;

        if d_vec.len() == 0 {
            panic!("Empty DVector")
        }
        else if d_vec.len() == 1 {
            return index
        }
        for curr_index in 1..d_vec.len() {
            if d_vec[curr_index] > d_vec[index] {
                index = curr_index;
            }
        }
        index
    }
}