use nalgebra::DVector;
use blazing_intelligence::layers::dense_layer::DenseLayer;
use blazing_intelligence::models::feed_forward::FeedForward;
use blazing_intelligence::models::learning_rate::base_learning_rate_adjusters::StepAdjust;
use blazing_intelligence::models::model::Model;
use blazing_intelligence::layers::activation_layer::ActivationLayer;
use blazing_intelligence::layers::activation_function::ActivationFunction;

fn main() {
    let mut feed_forward: Box<dyn Model> = FeedForward::new(
        vec![
            Box::new(DenseLayer::new(10,20)),
            Box::new(ActivationLayer::new(ActivationFunction::tanh())),
            Box::new(DenseLayer::new(20,20)),
            Box::new(ActivationLayer::new(ActivationFunction::tanh())),
            Box::new(DenseLayer::new(20,10)),
            Box::new(ActivationLayer::new(ActivationFunction::tanh())),
        ],
        Box::new(StepAdjust::new(0.0001, 0.01))
    );
    let output = feed_forward.prompt(DVector::new_random(10));
    print!("{}", output)
}