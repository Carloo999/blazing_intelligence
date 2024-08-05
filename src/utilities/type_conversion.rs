use nalgebra::{DMatrix, dmatrix, DVector, dvector, Scalar, VecStorage};

pub trait ToSavable<T: Clone + Sized>{
    type Output;

    fn to_savable(&self) -> Self::Output;
}

impl<T: Clone + Sized> ToSavable<T> for DVector<T>{
    type Output = Vec<T>;

    fn to_savable(&self) -> Self::Output{
        self.as_slice().to_vec()
    }
}

impl<T: Clone + Sized> ToSavable<T> for DMatrix<T>{
    type Output = Vec<Vec<T>>;

    fn to_savable(&self) -> Self::Output {
        self
            .as_slice()
            .chunks(self.ncols())
            .map(|row| row.to_vec())
            .collect()
    }
}

//-------------------------------------------------------------------------------


pub trait FromSavable<T: Clone + Sized + Scalar>{
    type Output;

    fn from_savable(&self) -> Self::Output;
}

impl<T: Clone + Sized + Scalar> FromSavable<T> for Vec<T>{
    type Output = DVector<T>;

    fn from_savable(&self) -> Self::Output{
        DVector::from_vec(self.clone())
    }
}

impl<T: Clone + Sized + Scalar> FromSavable<T> for Vec<Vec<T>>{
    type Output = DMatrix<T>;

    fn from_savable(&self) -> Self::Output{
        let flattened: Vec<T> = self.iter().flatten().cloned().collect();
        let rows:usize = self.len();
        let cols:usize = self[0].len();
        DMatrix::from_vec(rows, cols, flattened)
    }
}


