use crate::models::model::Model;
use crate::layers::layer::Layer;
struct FeedForward {
    layers: Vec<Box<dyn Layer>>,
    learning_rate: f64,
}

impl Model for FeedForward {
    fn prompt(&self) {
        todo!()
    }

    fn train(&self) {
        todo!()
    }
}