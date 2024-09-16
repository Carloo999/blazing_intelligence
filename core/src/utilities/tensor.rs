#[derive(Debug)]
pub struct Tensor {
    shape: Vec<usize>,
    data: Vec<f64>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Tensor { shape, data: vec![0.0; size] }
    }

    pub fn from_data(shape: Vec<usize>, data: Vec<f64>) -> Self {
        assert_eq!(shape.iter().product::<usize>(), data.len(),"Data does not match shape");
        Tensor { shape, data }
    }

    pub fn get(&self, indices: Vec<usize>) -> f64 {
        self.data[self.calc_index(&indices)]
    }

    pub fn set(&mut self, indices: Vec<usize>, val: f64) {
        let index = self.calc_index(&indices);
        self.data[index] = val;
    }

    pub fn reshape(&mut self, indices: Vec<usize>) {
        assert_eq!(indices.iter().product::<usize>(), self.data.len(),"Data does not match shape");
        self.shape = indices;
    }

    pub fn flatten(&mut self) {
        self.shape = vec![self.data.len()]
    }

    pub fn element_wise_mul(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "Tensor shapes must match");
        let data: Vec<f64> = self.data.iter().zip(other.data.iter())
            .map(|(a, b)| { a * b })
            .collect();
        Self::from_data(self.shape.clone(), data)
    }

    pub fn element_wise_div(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "Tensor shapes must match");
        let data: Vec<f64> = self.data.iter().zip(other.data.iter())
            .map(|(a, b)| { a / b })
            .collect();
        Self::from_data(self.shape.clone(), data)
    }

    pub fn matrix_vec_mul(&self, other: &Self) -> Self {
        assert_eq!(self.shape.len(), 2);
        assert_eq!(other.shape.len(), 1);

        let (m ,n) = (self.shape[0], self.shape[1]);
        let k = other.shape[0];

        assert_eq!(n, k);

        let mut result_data = vec![0.0; m];

        for i in 0..m {
            let mut sum = 0.0;
            for j in 0..k {
                sum += self.data[i * n + j] * other.data[j]
            }
            result_data[i] = sum;
        }
        Self::from_data(vec![m], result_data)
    }

    pub fn matrix_mul(&self, other: &Self) -> Self {
        assert_eq!(self.shape.len(), 2);
        assert_eq!(other.shape.len(), 2);

        let (m ,n) = (self.shape[0], self.shape[1]);
        let (l, k) = (other.shape[0], other.shape[1]);

        assert_eq!(n, l);

        let mut result_data = vec![0.0; m * k];

        for i in 0..m {
            for j in 0..k {
                let mut sum = 0.0;
                    for f in 0..n {
                        sum += self.data[i * n + f] * other.data[f * k + j];
                    }
                result_data[i * k + j] = sum;
            }
        }
        Self::from_data(vec![m , k], result_data)
    }

    pub fn dot(&self, other: &Self) -> Self {
        assert_eq!(self.shape.len(), 1);
        assert_eq!(other.shape.len(), 1);
        assert_eq!(self.shape[0], other.shape[0]);

        let sum = self.data.iter().zip(other.data.iter()).map(|(a, b)| { a * b }).sum();
        Tensor::from_data(vec![], vec![sum])
    }

    pub fn transpose(&self) -> Self {
        match self.shape.len() {
            1 => Tensor::from_data(self.shape.clone(), self.data.clone()),
            2 => self.mat_transpose(),
            _ => panic!("Higher dimension transpose is not implemented.")
        }
    }

    pub fn sum(&self, axis: Option<usize>) -> Self {
        todo!()
    }

    pub fn max(&self, axis: Option<usize>) -> Vec<usize> {
        todo!()
    }
}

impl Tensor {
    fn calc_index(&self, indices: &Vec<usize>) -> usize {
        assert_eq!(indices.len(), self.shape.len(), "Indices length must match shape of tensor");

        let mut index = 0;
        let mut stride = 1;

        for (i, &dim) in indices.iter().rev().enumerate() {
            index += dim * stride;
            stride *= self.shape[self.shape.len() - 1 - i];
        }
        index
    }

    fn mat_transpose(&self) -> Self {
        let mut new_data = self.data.clone();
        for i in 0..self.shape[0] {
            for j in 0..self.shape[1] {
                new_data[i * self.shape[1] + j] = self.get(vec![i, j])
            }
        }

        Tensor::from_data(vec![self.shape[1], self.shape[0]], new_data)
    }
}

impl std::ops::Add for Tensor {
    type Output = Self;

    fn add(self, other: Tensor) -> Self::Output {
        assert_eq!(self.shape, other.shape, "Tensor shapes must match");
        let data: Vec<f64> = self.data.iter().zip(other.data.iter())
            .map(|(a, b)| {a + b})
            .collect();
        Self::from_data(self.shape.clone(), data)
    }
}

impl std::ops::Sub for Tensor {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let data: Vec<f64> = self.data.iter().zip(other.data.iter())
            .map(|(a, b)| {a - b})
            .collect();
        Self::from_data(self.shape.clone(), data)
    }
}

impl std::ops::Mul<f64> for Tensor {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        let data: Vec<f64> = self.data.iter()
            .map(|&x| { x * scalar}).collect();

        Self::from_data(self.shape.clone(), data)
    }
}

impl std::ops::Mul<Self> for Tensor {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // Tensor mul?? "Dynamic" mul??
        todo!()
    }
}