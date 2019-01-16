use crate::dimension::{Dimension, D1};
use crate::numeric::Numeric;
use crate::matrix::Matrix;

pub trait Vector<T: Numeric, D: Dimension>: Matrix<T, D, D1> {
    fn get_size(&self) -> usize;
    fn norm(&self) -> T::BasicType;
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
}