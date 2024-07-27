use crate::models::learning_rate::base_learning_rate_adjusters::{StepAdjust, ExpAdjust};
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;

#[derive(Savefile)]
pub enum LearningRateAdjusterEnum {
    StepAdjust(StepAdjust),
    ExpAdjust(ExpAdjust),
}

impl LearningRateAdjusterEnum {
    pub fn convert_to_learning_rate_adjuster(&self) -> Box<dyn LearningRateAdjuster> {
        match self {
            LearningRateAdjusterEnum::StepAdjust(x) => return Box::new(x.clone()),
            LearningRateAdjusterEnum::ExpAdjust(x) => return Box::new(x.clone()),
        }
    }
}