use std::path::Path;
use nalgebra::DVector;
use savefile::{load_file, save_file, SavefileError};
use crate::nalgebra_utilities::type_conversion::{FromSavable, ToSavable};

pub struct Dataset {
    pub inputs: Vec<DVector<f64>>,
    pub labels: Vec<DVector<f64>>,
}

impl Dataset {
    fn size(&self) -> usize {
        self.labels.len()
    }

    fn new() -> Dataset {
        todo!()
    }

    pub fn save(&self, filepath: &Path) -> Result<(), SavefileError>{
        let mut savable_inputs: Vec<Vec<f64>> = vec![];
        let mut savable_outputs: Vec<Vec<f64>> = vec![];
        for dvector in self.inputs.iter(){
            savable_inputs.push(dvector.to_savable());
        }
        for dvector in self.labels.iter(){
            savable_outputs.push(dvector.to_savable());
        }
        save_file(filepath, 0, &(savable_inputs,savable_outputs))
    }

    pub fn load(filepath: &Path) -> Result<Dataset, SavefileError>{
        let loaded_savable: (Vec<Vec<f64>>, Vec<Vec<f64>>) = load_file(filepath, 0)?;
        let mut loaded_inputs:Vec<DVector<f64>>= loaded_savable.0.iter().map(|vec| vec.from_savable()).collect();
        let mut loaded_labels: Vec<DVector<f64>> = loaded_savable.1.iter().map(|vec| vec.from_savable()).collect();

        Ok(Dataset{
            inputs: loaded_inputs,
            labels: loaded_labels
        })


    }
}