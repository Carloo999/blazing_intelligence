use nalgebra::{DVector, dvector, DMatrix, dmatrix};

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

    pub fn conv_2d_valid(m1: &DMatrix<f64>, m2: &DMatrix<f64>) -> DMatrix<f64> {
        assert!(m1.shape().0 >= m2.shape().0 && m1.shape().1 >= m2.shape().1);

        let output_rows = m1.shape().0 - m2.shape().0 + 1;
        let output_cols = m1.shape().1 - m2.shape().1 + 1;
        let mut output_matrix: DMatrix<f64> = DMatrix::zeros(output_rows, output_cols);

        for offset_x in 0..output_rows {
            for offset_y in 0..output_cols {
                let mut sum = 0.0;
                for x in 0..m2.shape().0 {
                    for y in 0..m2.shape().1 {
                        sum += m1[(x + offset_x, y + offset_y)] * m2[(x, y)];
                    }
                }
                output_matrix[(offset_x, offset_y)] = sum;
            }
        }
        output_matrix
    }

    pub fn conv_2d_full(m1: DMatrix<f64>, m2: DMatrix<f64>) -> DMatrix<f64> {
        todo!()
    }
}