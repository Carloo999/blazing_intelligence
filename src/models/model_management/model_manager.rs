use std::io;
use std::path::Path;
use savefile::SavefileError;
use crate::models::model::{Model};
use crate::models::model_management::model_enum::ModelEnum;
use crate::savefile_derive;

pub trait ModelManager: ConvertToModelEnum {
    fn save(&self, path: &Path) -> Result<(), SavefileError>;
    fn load(filepath: &Path) -> Result<Box<dyn Model>, SavefileError>;
}


pub trait ConvertToModelEnum{
    fn convert_to_enum(&self) -> ModelEnum;
}