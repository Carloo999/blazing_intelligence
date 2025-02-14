use crate::models::learning_rate::learning_rate_adjuster::{ConvertToLearningRateAdjusterEnum, LearningRateAdjuster};
use crate::models::model_management::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
use crate::models::learning_rate::training_context::TrainingContext;


///To add a new learning rate adjuster create a new struct and implement the "LearningRateAdjuster" trait.
///Add the new struct in [learning_rate_adjuster_enum.rs](https://github.com/Carloo999/blazing_intelligence/blob/9f2572a47a001e5c91844be90686641102f2d52c/src/models/model_management/learning_rate_adjuster_enum.rs)
///to "LearningRateAdjusterEnum" and the match block in the "convert_to_learning_rate_adjuster()" function.

/// # ExpAdjust
/// Exponentialy decreases the learningrate
#[derive(Savefile, Clone)]
pub struct ExpAdjust {
    curr_learning_rate: f64
}

impl ExpAdjust {
    /// Constructs a new ExpAdjust with the given starting learning rate
    pub fn new(starting_learning_rate: f64) -> ExpAdjust {
        ExpAdjust{
            curr_learning_rate: starting_learning_rate,
        }
    }
}

impl LearningRateAdjuster for ExpAdjust {
    /// Decreases the learning rate exponentially
    fn adjust(&mut self, context: &mut TrainingContext){
        todo!()
    }

    /// Returns the current learning rate
    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}


impl ConvertToLearningRateAdjusterEnum for ExpAdjust {
    /// Converts the ExpAdjust to a LearningRateAdjusterEnum which is the savable type for learning rate adjusters
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::ExpAdjust(self.clone())
    }
}




/// # StepAdjust
/// Decreases the learning rate by a fixed step each epoch
#[derive(Savefile, Clone)]
pub struct StepAdjust {
    curr_learning_rate: f64,
    step_width: f64
}

impl StepAdjust {
    /// Constructs a new StepAdjust with the given step width and starting learning rate
    pub fn new(step_width: f64, starting_learning_rate: f64) -> StepAdjust {
        StepAdjust {
            curr_learning_rate: starting_learning_rate,
            step_width,
        }
    }
}

impl LearningRateAdjuster for StepAdjust {
    /// Decreases the learning rate by the step width
    fn adjust(&mut self, _context: &mut TrainingContext){
        self.curr_learning_rate -= self.step_width
    }

    /// Returns the current learning rate
    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

impl ConvertToLearningRateAdjusterEnum for StepAdjust{
    /// Converts the StepAdjust to a LearningRateAdjusterEnum which is the savable type for learning rate adjusters
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::StepAdjust(self.clone())
    }
}



///# DynAdjust
/// Multiplies the learning rate by a reduction factor if the mse increases
#[derive(Savefile, Clone)]
pub struct DynAdjust {
    curr_learning_rate: f64,
    reduction_factor: f64,
}

impl DynAdjust {
    /// Constructs a new DynAdjust with the given starting learning rate and reduction factor
    pub fn new(starting_learning_rate: f64, reduction_factor: f64) -> DynAdjust {
        DynAdjust {
            curr_learning_rate: starting_learning_rate,
            reduction_factor,
        }
    }
}

impl ConvertToLearningRateAdjusterEnum for DynAdjust {
    /// Converts the DynAdjust to a LearningRateAdjusterEnum which is the savable type for learning rate adjusters
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
        LearningRateAdjusterEnum::DynAdjust(self.clone())
    }
}


impl LearningRateAdjuster for DynAdjust {
    /// Multiplies the learning rate by the reduction factor if the mse increases
    fn adjust(&mut self, context: &mut TrainingContext){
        if context.mse_evolution[context.mse_evolution.len()-2] < context.mse_evolution[context.mse_evolution.len()-1]{
            self.curr_learning_rate *= self.reduction_factor;
        }
    }
    /// Returns the current learning rate
    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

///# Static Adjust
/// Does not change the learning rate
/// not practical but useful for testing
#[derive(Savefile, Clone)]
pub struct StaticAdjust {
    static_learning_rate: f64,
}

impl StaticAdjust {
    /// Constructs a new StaticAdjust with the given static learning rate
    pub fn new(static_learning_rate: f64) -> StaticAdjust {
        StaticAdjust {
            static_learning_rate,
        }
    }
}

impl LearningRateAdjuster for StaticAdjust {
    /// Does not change the learning rate
    fn adjust(&mut self, _context: &mut TrainingContext) { }

    /// Returns the current learning rate
    fn get_learning_rate(&self) -> f64 {
        self.static_learning_rate
    }
}

impl ConvertToLearningRateAdjusterEnum for StaticAdjust {
    /// Converts the StaticAdjust to a LearningRateAdjusterEnum which is the savable type for learning rate adjusters
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum {
    LearningRateAdjusterEnum::StaticAdjust(self.clone())
    }
}