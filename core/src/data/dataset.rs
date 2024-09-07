use std::io;
use csv::Reader;
use nalgebra::{DVector, dvector};
use std::path::Path;
use savefile::{load_file, save_file, SavefileError};
use crate::utilities::type_conversion::{FromSavable, ToSavable};

#[derive(Clone)]
pub struct Dataset {
    pub inputs: Vec<DVector<f64>>,
    pub labels: Vec<DVector<f64>>
}

impl Dataset {
    pub fn size(&self) -> usize {
        self.labels.len()
    }

    pub fn new(inputs: Vec<DVector<f64>>, labels: Vec<DVector<f64>>) -> Dataset {
        Dataset {
            inputs,
            labels,
        }
    }

    pub fn new_from_csv(file_path: &str, output_possibilities: usize) -> Result<Dataset, io::Error> {
        let mut reader = Reader::from_path(file_path)?;
        let mut inputs: Vec<DVector<f64>> = Vec::new();
        let mut labels: Vec<DVector<f64>> = Vec::new();

        for result in reader.records() {
            let record = result?;
            let mut record_iter = record.iter();

            if let Some(label_val) = record_iter.next() {
                let label = label_val.parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid label value"))?;
                labels.push(Self::get_expected_probabilities_vec(output_possibilities, label as usize));
            }

            let input: DVector<f64> = DVector::from_vec(record_iter
                .map(|val| val.parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid input value")))
                .collect::<Result<_, _>>()?);

            inputs.push(input);
        }

        Ok(Dataset { inputs, labels })
    }
    fn get_expected_probabilities_vec(possibilities: usize, correct_option: usize) -> DVector<f64> {
        let mut vec = vec![0.0; possibilities];
        vec[correct_option] = 1.0;
        DVector::from_vec(vec)
    }

    pub fn save(&self, filepath: &Path) -> Result<(), SavefileError> {
        let mut savable_inputs: Vec<Vec<f64>> = vec![];
        let mut savable_outputs: Vec<Vec<f64>> = vec![];
        for dvector in self.inputs.iter() {
            savable_inputs.push(dvector.to_savable());
        }
        for dvector in self.labels.iter() {
            savable_outputs.push(dvector.to_savable());
        }
        save_file(filepath, 0, &(savable_inputs, savable_outputs))
    }

    pub fn load(filepath: &Path) -> Result<Dataset, SavefileError> {
        let loaded_savable: (Vec<Vec<f64>>, Vec<Vec<f64>>) = load_file(filepath, 0)?;
        let mut loaded_inputs: Vec<DVector<f64>> = loaded_savable.0.iter().map(|vec| vec.from_savable()).collect();
        let mut loaded_labels: Vec<DVector<f64>> = loaded_savable.1.iter().map(|vec| vec.from_savable()).collect();

    Ok(Dataset{
        inputs: loaded_inputs,
        labels: loaded_labels
    })
    }
}