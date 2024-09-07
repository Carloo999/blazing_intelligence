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

    pub fn reshape(&self, indices: Vec<usize>) {
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
}

impl std::ops::Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
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
        todo!()
    }
}