use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
pub trait LearningRateAdjuster: ConvertToLearningRateAdjusterEnum{
    fn adjust(&mut self, context: &mut TrainingContext);
    fn get_learning_rate(&self) -> f64;
}

pub trait ConvertToLearningRateAdjusterEnum{
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum;
}
