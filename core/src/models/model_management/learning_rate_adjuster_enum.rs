use crate::models::learning_rate::base_learning_rate_adjusters::{StepAdjust, ExpAdjust, DynAdjust, StaticAdjust};
use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;

#[derive(Savefile)]
pub enum LearningRateAdjusterEnum {
    StepAdjust(StepAdjust),
    ExpAdjust(ExpAdjust),
    DynAdjust(DynAdjust),
    StaticAdjust(StaticAdjust),
}

impl LearningRateAdjusterEnum {
    pub fn convert_to_learning_rate_adjuster(&self) -> Box<dyn LearningRateAdjuster> {
        match self {
            LearningRateAdjusterEnum::StepAdjust(x) => Box::new(x.clone()),
            LearningRateAdjusterEnum::ExpAdjust(x) => Box::new(x.clone()),
            LearningRateAdjusterEnum::DynAdjust(x) => Box::new(x.clone()),
            LearningRateAdjusterEnum::StaticAdjust(x) => Box::new(x.clone()),
        }
    }
}