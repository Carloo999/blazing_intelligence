use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
/// LearningRateAdjuster trait to be implemented by every type of learning rate adjuster
pub trait LearningRateAdjuster: ConvertToLearningRateAdjusterEnum{
    /// Adjusts the learning rate
    fn adjust(&mut self, context: &mut TrainingContext);
    /// Returns the current learning rate
    fn get_learning_rate(&self) -> f64;
}

/// ConvertToLearningRateAdjusterEnum trait to be implemented by learning rate adjusters that can be converted to LearningRateAdjusterEnum to be saved to a file
pub trait ConvertToLearningRateAdjusterEnum{
    /// Converts the learning rate adjuster to a LearningRateAdjusterEnum which is the savable type for learning rate adjusters
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum;
}
