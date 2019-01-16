pub use num_complex::Complex32;
pub use num_complex::Complex64;

pub enum PrimitiveType {
    Float32,
    Float64
}

pub enum Type {
    Basic(PrimitiveType),
    Complex(PrimitiveType)
}

pub trait Numeric {
    fn static_type() -> Type;
}

impl Numeric for f32 {
    fn static_type() -> Type {
        Type::Basic(PrimitiveType::Float32)
    }
}

impl Numeric for f64 {
    fn static_type() -> Type {
        Type::Basic(PrimitiveType::Float64)
    }
}

impl Numeric for Complex32 {
    fn static_type() -> Type {
        Type::Complex(PrimitiveType::Float32)
    }
}

impl Numeric for Complex64 {
    fn static_type() -> Type {
        Type::Complex(PrimitiveType::Float64)
    }
}