use std::ops::*;
use std::cmp::*;
mod ops;
mod float;
pub use ops::{NumOps, RealOps};
use std::fmt;
use std::fmt::Display;


/// A wrapper type for numerics that holds additional information about the maximum possible
/// deviation from the correct value. Makes it possible to equality test without an epsilon.
#[derive(Debug, Copy, Clone)]
pub struct S<T> {
    value: T,
    width: T
}

impl <T: Display> Display for S<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+-{}", &self.value, &self.width)
    }
}

impl <T: NumOps> S<T> {
    pub fn wide_zero(width: T) -> Self {
        S {
            value: T::zero(),
            width: width,
        }
    }
    pub fn thin_zero() -> Self {
        S {
            value: T::zero(),
            width: T::zero()
        }
    }
    pub fn exact(num: T) -> Self {
        S {
            value: num,
            width: T::zero(),
        }
    }
}

impl <T: NumOps + Copy> S<T> {
    pub fn s(num: T) -> Self {
        S {
            value: num,
            width: num.epsilon()
        }
    }
}


impl <T: PartialOrd + Add<Output=T> + Sub<Output=T> + NumOps + Copy>  S<T> {
    pub fn could_eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() <= self.width + other.width
    }
}

/*impl <T: PartialOrd + Add + Sub> PartialOrd for S<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value + self.width < other.value - other.width {
            Some(Ordering::Less)
        } else if self.value - self.width > other.value - other.width {
            Some(Ordering::Greater)
        }
        None
    }
}*/

impl <T:RealOps + NumOps + Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>> S<T> {
    pub fn lerp(self, other: Self, t: Self) -> Self {
        self * (Self::exact(T::one()) - t) + other * t
    }
    pub fn from_bounds(bottom: T, top: T) -> Self {
        let mid = (bottom + top) / (T::one() + T::one());
        let width = (mid - bottom).max(top - mid);
        S {
            value: mid,
            width: width
        }
    }
    pub fn abs(self) -> Self {
        S {
            value: self.value.abs(),
            width: self.width
        }.max(Self::thin_zero())
    }
}

impl <T:RealOps + NumOps + Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>> RealOps for S<T> {
    fn sin(self) -> Self {
        S {
            value: self.value.sin(),
            width: self.width + self.value.epsilon()
        }
    }
    fn cos(self) -> Self {
        S {
            value: self.value.cos(),
            width: self.width + self.value.epsilon()
        }
    }
    fn pi() -> Self {
        S::s(T::pi())
    }
    fn e() -> Self {
        S::s(T::e())
    }
    fn max(self, other: Self) -> Self {
        let bottom = (self.value - self.width).max(other.value - other.width);
        let top = (self.value + self.width).max(other.value + other.width);
        Self::from_bounds(bottom, top)
    }
    fn min(self, other: Self) -> Self {
        let bottom = (self.value - self.width).min(other.value - other.width);
        let top = (self.value + self.width).min(other.value + other.width);
        Self::from_bounds(bottom, top)
    }
}


impl <T> Add for S<T> where T: Add {
    type Output = S<T::Output>;
    fn add(self, other: Self) -> Self::Output {
        S {
            value: self.value + other.value,
            width: self.width + other.width
        }
    }
}


impl <T> Sub for S<T> where T: Add<Output=T> + Sub<Output=T> {
    type Output = S<T>;
    fn sub(self, other: Self) -> Self::Output {
        S {
            value: self.value - other.value,
            width: self.width + other.width
        }
    }
}

impl <T> Mul for S<T> where T: Add<Output=T> + Mul<Output=T> + NumOps + Copy{
    type Output = S<T>;
    fn mul(self, other: Self) -> Self::Output {
        S {
            value: self.value * other.value,
            width: (self.width * other.value).abs() + (other.width * self.value).abs(),
        }
    }
}

impl <T> Div for S<T> where T: Div<Output=T> + Mul<Output=T> {
    type Output = S<T>;
    fn div(self, other: Self) -> Self::Output {
        S {
            // TODO fix
            value: self.value / other.value,
            width: self.width * other.width
        }
    }
}
