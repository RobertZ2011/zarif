use crate::numeric::Numeric;
use crate::dimension::Dimension;
use std::ops::{
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
    Neg
};

pub trait Matrix<T: Numeric, N: Dimension, M: Dimension>:
    Sized +
    Add +
    AddAssign +
    Sub +
    SubAssign +
    Mul +
    Mul<T> +
    MulAssign +
    MulAssign<T> +
    Neg {
    fn get_row_count(&self) -> usize;
    fn get_col_count(&self) -> usize; 
}