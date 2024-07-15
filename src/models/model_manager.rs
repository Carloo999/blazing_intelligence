use std::io;
use std::path::Path;
use crate::models::model::Model;

trait ModelManager {
    fn save(model: dyn Model) -> Result<(), io::Error>;
    fn load(filepath: Path) -> Result<Box<dyn Model>, io::Error>;
}