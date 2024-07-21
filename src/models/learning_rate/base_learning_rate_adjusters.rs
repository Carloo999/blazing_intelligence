use crate::models::learning_rate::learning_rate_adjuster::LearningRateAdjuster;
use crate::models::learning_rate::training_context::TrainingContext;


struct StepAdjust {
    curr_learning_rate: f64
}

impl LearningRateAdjuster for StepAdjust {
    fn adjust(&mut self, context: TrainingContext) {
        todo!()
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}

struct ExpAdjust {
    curr_learning_rate: f64
}

impl LearningRateAdjuster for ExpAdjust {
    fn adjust(&mut self, context: TrainingContext){
        todo!()
    }

    fn get_learning_rate(&self) -> f64 {
        self.curr_learning_rate
    }
}