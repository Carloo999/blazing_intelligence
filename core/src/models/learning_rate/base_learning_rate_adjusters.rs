use crate::models::learning_rate::learning_rate_adjuster::{ConvertToLearningRateAdjusterEnum, LearningRateAdjuster};
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
use crate::models::learning_rate::training_context::TrainingContext;


///To add a new learning rate adjuster create a new struct and implement the "LearningRateAdjuster" trait.
///Add the new struct in [learning_rate_adjuster_enum.rs](https://github.com/Carloo999/blazing_intelligence/blob/9f2572a47a001e5c91844be90686641102f2d52c/src/models/model_management/learning_rate_adjuster_enum.rs)
///to "LearningRateAdjusterEnum" and the match block in the "convert_to_learning_rate_adjuster()" function.

/// # ExpAdjust
#[derive(Savefile, Clone)]
pub struct ExpAdjust {
    curr_learning_rate: f64
}

impl ExpAdjust {
    pub fn new(starting_learning_rate: f64) -> ExpAdjust {
        ExpAdjust{
            curr_learning_rate: starting_learning_rate,
        }
    }
}

impl LearningRateAdjuster for ExpAdjust {
    fn adjust(&mut self, context: &mut TrainingContext){
        todo!()
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}


impl ConvertToLearningRateAdjusterEnum for ExpAdjust {
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::ExpAdjust(self.clone())
    }
}




/// # StepAdjust
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
    fn adjust(&mut self, _context: &mut TrainingContext){
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



///# DynAdjust
#[derive(Savefile, Clone)]
pub struct DynAdjust {
    curr_learning_rate: f64,
    reduction_factor: f64,
}

impl DynAdjust {
    pub fn new(starting_learning_rate: f64, reduction_factor: f64) -> DynAdjust {
        DynAdjust {
            curr_learning_rate: starting_learning_rate,
            reduction_factor,
        }
    }
}

impl ConvertToLearningRateAdjusterEnum for DynAdjust {
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::DynAdjust(self.clone())
    }
}


impl LearningRateAdjuster for DynAdjust {
    fn adjust(&mut self, context: &mut TrainingContext){
        if context.mse_evolution[context.mse_evolution.len()-2] < context.mse_evolution[context.mse_evolution.len()-1]{
            self.curr_learning_rate *= self.reduction_factor;
        }
    }
    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

///# Static Adjust
#[derive(Savefile, Clone)]
pub struct StaticAdjust {
    static_learning_rate: f64,
}

impl StaticAdjust {
    pub fn new(static_learning_rate: f64) -> StaticAdjust {
        StaticAdjust {
            static_learning_rate,
        }
    }
}

impl LearningRateAdjuster for StaticAdjust {
    fn adjust(&mut self, _context: &mut TrainingContext) { }

    fn get_learning_rate(&self) -> f64 {
        self.static_learning_rate
    }
}

impl ConvertToLearningRateAdjusterEnum for StaticAdjust {
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
    LearningRateAdjusterEnum::StaticAdjust(self.clone())
    }
}