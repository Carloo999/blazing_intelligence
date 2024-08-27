use nalgebra::{DMatrix, DVector};
use crate::layers::conv_shape::ConvShape;
use crate::layers::layer::{BackwardPropagationStochastic, ForwardPropagation};
use crate::utilities::math_utils::MathUtils;

pub struct ConvolutionalLayer {
    depth: usize,
    
    input_shape: ConvShape,
    output_shape: ConvShape,
    kernel_shape: ConvShape,

    kernels: Vec<Vec<DMatrix<f64>>>,
    biases: Vec<DMatrix<f64>>,
}

impl ForwardPropagation for ConvolutionalLayer {

    fn forwards_propagate(&mut self, input: Vec<DMatrix<f64>>) -> Vec<DMatrix<f64>> {
        let output = self.biases.clone();

        for i in 0..self.depth {
            for j in 0..self.input_shape.depth {
                output[i] += MathUtils::conv_2d_valid(&input[j], &self.kernels[(i, j)])
            }
        }

        output
    }
}

impl BackwardPropagationStochastic for ConvolutionalLayer {

    fn backwards_propagate(&mut self, output_grad: &DVector<f64>, learning_rate: &f64) -> DVector<f64> {
        todo!()
    }
}

impl ConvolutionalLayer {
    pub fn new(input_shape: ConvShape, kernel_size: usize, depth: usize) -> ConvolutionalLayer {
        let output_shape = ConvShape::new(depth, input_shape.height - kernel_size + 1, input_shape.width - kernel_size + 1);
        let kernel_shape = ConvShape::new(depth, input_shape.depth, kernel_size);

        let mut kernels: Vec<Vec<DMatrix<f64>>> = vec![];
        let mut biases: Vec<DMatrix<f64>> = vec![];

        for _ in 0..depth {
            let mut kernel_row: Vec<DMatrix<f64>> = vec![];
            for _ in 0..input_shape.depth {
                kernel_row.push(DMatrix::zeros(kernel_size, kernel_size));
            }
            kernels.push(kernel_row);
            biases.push(DMatrix::zeros(output_shape.height, output_shape.width));
        }

        ConvolutionalLayer {
            depth,
            
            input_shape,
            output_shape,
            kernel_shape,

            kernels,
            biases,
        }
    }
}