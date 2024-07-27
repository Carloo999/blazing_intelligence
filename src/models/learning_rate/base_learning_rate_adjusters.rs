use crate::models::learning_rate::learning_rate_adjuster::{ConvertToLearningRateAdjusterEnum, LearningRateAdjuster};
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
use crate::models::learning_rate::training_context::TrainingContext;

#[derive(Savefile, Clone)]
pub(crate) struct ExpAdjust {
    curr_learning_rate: f64
}

impl ExpAdjust {
    pub fn new() -> ExpAdjust {
        todo!()
    }
}

impl LearningRateAdjuster for ExpAdjust {
    fn adjust(&mut self, context: TrainingContext) {
        todo!()
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

impl ConvertToLearningRateAdjusterEnum for ExpAdjust {
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        todo!()
    }
}

impl ConvertToLearningRateAdjusterEnum for ExpAdjust {
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::ExpAdjust(self.clone())
    }
}

#[derive(Savefile, Clone)]
pub struct StepAdjust {
    curr_learning_rate: f64,
    step_width: f64
}

impl StepAdjust {
    pub fn new(step_width: f64, starting_learning_rate: f64) -> StepAdjust {
        StepAdjust {
            curr_learning_rate: starting_learning_rate,
            step_width,
        }
    }
}

impl LearningRateAdjuster for StepAdjust {
    fn adjust(&mut self, context: TrainingContext){
        self.curr_learning_rate -= self.step_width
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

impl ConvertToLearningRateAdjusterEnum for StepAdjust{
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::StepAdjust(self.clone())
    }
}

