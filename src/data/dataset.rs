use nalgebra::DVector;

pub struct Dataset {
    pub inputs: DVector<DVector<f64>>,
    pub labels: DVector<f64>
}

impl Dataset {
    fn size(&self) -> usize {
        self.labels.len()
    }

    fn new() -> Dataset {
        todo!()
    }

}