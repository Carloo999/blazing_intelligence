use std::io;
use std::path::Path;
use crate::models::model::{Model};
use crate::models::model_enum::ModelEnum;


trait ModelManager: ConvertToModelEnum {
    fn save(model: dyn Model) -> Result<(), io::Error>;
    fn load(filepath: Path) -> Result<Box<dyn Model>, io::Error>;
}


pub trait ConvertToModelEnum{
    fn convert_to_enum(&self) -> ModelEnum;
}