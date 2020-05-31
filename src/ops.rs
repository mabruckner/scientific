use crate::float::*;
use std::f64;

pub trait NumOps {
    fn abs(self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn max_val() -> Self;
    fn epsilon(self) -> Self;
}

impl NumOps for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn epsilon(self) -> Self {
        (next_f64(self) - self).abs()
    }
    fn max_val() -> Self {
        inf_f64()
    }
}

pub trait RealOps {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn pi() -> Self;
    fn e() -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    /*fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;*/
}

impl RealOps for f64 {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn pi() -> Self {
        f64::consts::PI
    }
    fn e() -> Self {
        f64::consts::E
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
