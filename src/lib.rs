//! # Fixed-point numbers
//!
//! fixed-point numbers use constant D as decimal digit length
//! e.g. `fixed!(1.1i16, 2)` will declare number as `110i16`
//!
//! * Define a constant fixed-point number
//!
//!   ```
//!   use fixed_point::{fixed, FixedPoint};
//!   const FIXED_POINT: FixedPoint<u16, 3> = fixed!(0.25, 3);
//!   ```
//!
//! * Define a fixed-point variable
//!
//!   ```
//!   use fixed_point::fixed;
//!   let decimal = fixed!(-1.1i16, 2);
//!   ```
//!
//! * Define a implicit precision fixed-point variable
//!
//!   ```
//!   use fixed_point::fixed;
//!   let decimal = fixed!(-1.1i16);
//!   ```

#![cfg_attr(not(any(test, feature = "std")), no_std)]

/// Define a fixed-point number
pub use macros::fixed;

use core::{convert, fmt::Display, ops, str::FromStr};
#[cfg(all(feature = "serde", not(any(test, feature = "std"))))]
use num_traits::float::FloatCore;
use num_traits::pow::Pow;

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct FixedPoint<T, const D: u8>(pub T);

impl<T, const D: u8> FixedPoint<T, D> {
    pub fn decimal_length(self) -> u8 {
        D
    }

    pub fn exp(self) -> usize {
        10_usize.pow(D as u32)
    }
}

pub trait Number {
    fn ten() -> Self;
    fn zero() -> Self;
}

macro_rules! impl_number {
    ($($types:ty),+) => {
        $(
            impl Number for $types {
                fn ten() -> Self {
                    10
                }

                fn zero() -> Self {
                    0
                }
            }
        )+
    };
}

impl_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl<T, const D: u8> FixedPoint<T, D>
where
    T: Number + Pow<u8, Output = T> + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    pub fn new(number: T, decimal: u8) -> Self {
        Self(number * T::ten().pow(D - decimal))
    }
}

impl<T, const D: u8> FixedPoint<T, D>
where
    T: Copy + Number + Pow<u32, Output = T> + ops::Div<Output = T> + ops::Rem<Output = T>,
{
    pub fn integer(&self) -> T {
        self.0 / (T::ten()).pow(D as u32)
    }

    pub fn decimal(&self) -> T {
        self.0 % (T::ten()).pow(D as u32)
    }
}

impl<T: ops::Div<Output = T>, const D: u8> ops::Div<T> for FixedPoint<T, D> {
    type Output = Self;

    fn div(self, div: T) -> Self {
        Self(self.0 / div)
    }
}

impl<T: Copy + Into<i32>, const D: u8> Into<f32> for FixedPoint<T, D> {
    fn into(self) -> f32 {
        let value: i32 = self.0.into();
        value as f32 / self.exp() as f32
    }
}

impl<T: convert::TryFrom<isize>, const D: u8> FromStr for FixedPoint<T, D> {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let negative = string.chars().next().map(|c| c == '-').unwrap_or(false);
        let mut splitted = string.split('.');
        let mut integer = splitted
            .next()
            .ok_or(())?
            .parse::<isize>()
            .map_err(|_| ())?;
        integer *= (10 as isize).pow(D as u32);
        let field = match splitted.next() {
            Some(s) => s,
            None => return T::try_from(integer).map(|v| Self(v)).map_err(|_| ()),
        };
        let decimal_length = core::cmp::min(field.len(), 255) as u8;
        let mut decimal = field.parse::<isize>().map_err(|_| ())?;
        if integer < 0 || negative {
            decimal = -decimal
        }
        if D >= decimal_length {
            decimal *= (10 as isize).pow((D - decimal_length) as u32);
        } else {
            decimal /= (10 as isize).pow((decimal_length - D) as u32);
        }
        T::try_from(integer + decimal)
            .map(|v| Self(v))
            .map_err(|_| ())
    }
}

impl<T, const D: u8> Display for FixedPoint<T, D>
where
    T: Copy
        + Display
        + Into<i32>
        + PartialEq
        + Number
        + PartialOrd
        + Pow<u32, Output = T>
        + ops::Div<Output = T>
        + ops::Rem<Output = T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut decimal = self.decimal().into().abs();
        if D == 0 || decimal == 0 {
            return write!(f, "{}.0", self.integer());
        }
        let mut length = D;
        while decimal % 10 == 0 {
            decimal = decimal / 10;
            length -= 1;
        }
        let integer = self.integer();
        if integer == T::zero() && self.0 < T::zero() {
            write!(f, "-0.{:0length$}", decimal, length = length as usize)
        } else {
            write!(
                f,
                "{}.{:0length$}",
                integer,
                decimal,
                length = length as usize
            )
        }
    }
}

#[cfg(feature = "serde")]
impl<T: Copy + Into<i32>, const D: u8> serde::Serialize for FixedPoint<T, D> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f32((*self).into())
    }
}

#[cfg(feature = "serde")]
impl<'a, T: convert::TryFrom<isize>, const D: u8> serde::Deserialize<'a> for FixedPoint<T, D> {
    fn deserialize<DE: serde::Deserializer<'a>>(deserializer: DE) -> Result<Self, DE::Error> {
        let float = <f32>::deserialize(deserializer)?;
        let v = (float * 10f32.powi(D as i32)) as isize;
        T::try_from(v)
            .map(|v| Self(v))
            .map_err(|_| <DE::Error as serde::de::Error>::custom("Not fixed-point"))
    }
}
