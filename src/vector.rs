use crate::dimension::Dimension;
use crate::numeric::Numeric;
use std::ops::{
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
    Neg
};

pub trait Vector<T: Numeric, D: Dimension>:
    Sized +
    Add +
    AddAssign +
    Sub +
    SubAssign +
    Mul +
    Mul<T> +
    MulAssign +
    MulASsign<T> +
    Neg {
    fn get_size(&self) -> usize;
}