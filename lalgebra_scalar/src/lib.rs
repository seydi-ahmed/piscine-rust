use core::ops::{Add , Sub , Mul};

pub trait Scalar: Add<Self , Output= Self> + Sized + Sub<Self , Output= Self>  + Mul<Self , Output= Self> {

    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}
