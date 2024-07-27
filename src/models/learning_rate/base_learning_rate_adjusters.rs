use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::learning_rate::training_context::TrainingContext;


struct ExpAdjust {
    curr_learning_rate: f64
}

impl LearningRateAdjuster for ExpAdjust {
    fn adjust(&mut self, context: TrainingContext) {
        todo!()
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

impl ExpAdjust {
    pub fn new() -> ExpAdjust {
        todo!()
    }
}

pub struct StepAdjust {
    curr_learning_rate: f64,
    step_width: f64
}

impl LearningRateAdjuster for StepAdjust {
    fn adjust(&mut self, context: TrainingContext){
        self.curr_learning_rate -= self.step_width
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

impl StepAdjust {
    pub fn new(step_width: f64, starting_learning_rate: f64) -> StepAdjust {
        StepAdjust {
            curr_learning_rate: starting_learning_rate,
            step_width,
        }
    }
}