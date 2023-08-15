use std::ops::{
    Add,
    AddAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

use crate::Reset;

#[cfg(feature = "f64")]
pub type Fp = f64;
#[cfg(not(feature = "f64"))]
pub type Fp = f32;

pub trait Zero {
    fn zero() -> Self;
}

#[inline]
#[must_use]
pub fn zero<T: Zero>() -> T {
    T::zero()
}

macro_rules! impl_zero {
    ($($Typ:ident)*) => {
        $(
            impl Zero for $Typ {
                fn zero() -> Self {
                    0
                }
            }

        )*

    };
}

impl_zero!(u8 u16 u32 u64 usize);
impl_zero!(i8 i16 i32 i64 isize);

macro_rules! impl_zero_float {
    ($($Typ:ident)*) => {
        $(
            impl Zero for $Typ {
                fn zero() -> Self {
                    0.
                }
            }

        )*

    };
}

impl_zero_float!(f32 f64);

pub trait One {
    fn one() -> Self;
}

#[inline]
#[must_use]
pub fn one<T: One>() -> T {
    T::one()
}

macro_rules! impl_one {
    ($($Typ:ident)*) => {
        $(
            impl One for $Typ {
                fn one() -> Self {
                    1
                }
            }

        )*

    };
}

impl_one!(u8 u16 u32 u64 usize);
impl_one!(i8 i16 i32 i64 isize);

macro_rules! impl_one_float {
    ($($Typ:ident)*) => {
        $(
            impl One for $Typ {
                fn one() -> Self {
                    1.
                }
            }

        )*

    };
}
impl_one_float!(f32 f64);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Bit {
    #[default]
    Zero,
    One,
}

impl Bit {
    #[must_use]
    pub fn is_one(&self) -> bool {
        match self {
            Bit::Zero => false,
            Bit::One => true,
        }
    }

    #[must_use]
    pub fn flip(self) -> Self {
        use Bit::{
            One,
            Zero,
        };
        match self {
            Zero => One,
            One => Zero,
        }
    }
}

impl Zero for Bit {
    fn zero() -> Self {
        Bit::Zero
    }
}

impl One for Bit {
    fn one() -> Self {
        Bit::One
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value.is_one()
    }
}

macro_rules! impl_from_bit {
    ($($Typ:ident)*) => {
        $(
            impl From<Bit> for $Typ {
                fn from(value: Bit) -> $Typ {
                    match value {
                        Bit::Zero => 0,
                        Bit::One => 1,
                    }
                }
            }

        )*

    };
}

impl_from_bit!(u8 u16 u32 u64 usize);
impl_from_bit!(i8 i16 i32 i64 isize);

macro_rules! impl_from_bit_float {
    ($($Typ:ident)*) => {
        $(
            impl From<Bit> for $Typ {
                fn from(value: Bit) -> $Typ {
                    match value {
                        Bit::Zero => 0.,
                        Bit::One => 1.,
                    }
                }
            }

        )*

    };
}
impl_from_bit_float!(f32 f64);

impl Reset for Bit {
    fn reset(&mut self) {
        *self = zero();
    }
}

pub trait Float:
    Default
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign<Self>
    + Neg<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign<Self>
    + Zero
    + One
{
    const EPSILON: Self;

    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_float<T: Float>(value: T) -> Self {
        Self::from_f64(value.to_f64())
    }

    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
    fn to_float<T: Float>(self) -> T {
        T::from_f64(self.to_f64())
    }
}

#[inline]
#[must_use]
pub fn two<T: One + Float>() -> T {
    one::<T>() + one::<T>()
}

impl Float for f32 {
    const EPSILON: Self = std::f32::EPSILON;

    fn from_f32(value: f32) -> Self {
        value
    }

    #[allow(clippy::cast_possible_truncation)]
    fn from_f64(value: f64) -> Self {
        value as Self
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    fn from_float<T: Float>(value: T) -> Self {
        value.to_f32()
    }

    fn to_float<T: Float>(self) -> T {
        T::from_f32(self)
    }
}

impl Float for f64 {
    const EPSILON: Self = std::f64::EPSILON;

    fn from_f32(value: f32) -> Self {
        f64::from(value)
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    #[allow(clippy::cast_possible_truncation)]
    fn to_f32(self) -> f32 {
        self as f32
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn from_float<T: Float>(value: T) -> Self {
        value.to_f64()
    }

    fn to_float<T: Float>(self) -> T {
        T::from_f64(self)
    }
}

pub trait Real: Float {
    const SQRT_2: Self;
    const E: Self;
    const LN_2: Self;
    const LN_10: Self;
    const PI: Self;
    const TAU: Self;

    #[must_use]
    fn recip(self) -> Self;
    #[must_use]
    fn floor(self) -> Self;
    #[must_use]
    fn ceil(self) -> Self;
    #[must_use]
    fn abs(self) -> Self;
    #[must_use]
    fn signum(self) -> Self;
    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn powf(
        self,
        n: Self,
    ) -> Self;
    #[must_use]
    fn exp(self) -> Self;
    #[must_use]
    fn exp2(self) -> Self;
    #[must_use]
    fn ln(self) -> Self;
    #[must_use]
    fn log(
        self,
        base: Self,
    ) -> Self;
    #[must_use]
    fn log2(self) -> Self;
    #[must_use]
    fn log10(self) -> Self;
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn tan(self) -> Self;
    #[must_use]
    fn asin(self) -> Self;
    #[must_use]
    fn acos(self) -> Self;
    #[must_use]
    fn atan(self) -> Self;
    #[must_use]
    fn sinh(self) -> Self;
    #[must_use]
    fn cosh(self) -> Self;
    #[must_use]
    fn tanh(self) -> Self;
    #[must_use]
    fn asinh(self) -> Self;
    #[must_use]
    fn acosh(self) -> Self;
    #[must_use]
    fn atanh(self) -> Self;
}

#[must_use]
pub const fn tau<T: Real>() -> T {
    T::TAU
}

#[must_use]
pub const fn sqrt2<T: Real>() -> T {
    T::SQRT_2
}

#[must_use]
pub fn one_half<T: One + Real>() -> T {
    two::<T>().recip()
}

macro_rules! impl_real {
    ($($Typ:ident)*) => {
        $(
            impl Real for $Typ {
                const E: Self = std::$Typ::consts::E;
                const LN_10: Self = std::$Typ::consts::LN_10;
                const LN_2: Self = std::$Typ::consts::LN_2;
                const PI: Self = std::$Typ::consts::PI;
                const SQRT_2: Self = std::$Typ::consts::SQRT_2;
                const TAU: Self = std::$Typ::consts::TAU;

                fn recip(self) -> Self {
                    self.recip()
                }

                fn floor(self) -> Self {
                    self.floor()
                }

                fn ceil(self) -> Self {
                    self.ceil()
                }

                fn abs(self) -> Self {
                    self.abs()
                }

                fn signum(self) -> Self {
                    self.signum()
                }

                fn sqrt(self) -> Self {
                    self.sqrt()
                }

                fn powf(
                    self,
                    n: Self,
                ) -> Self {
                    self.powf(n)
                }

                fn exp(self) -> Self {
                    self.exp()
                }

                fn exp2(self) -> Self {
                    self.exp2()
                }

                fn ln(self) -> Self {
                    self.ln()
                }

                fn log(
                    self,
                    base: Self,
                ) -> Self {
                    self.log(base)
                }

                fn log2(self) -> Self {
                    self.log2()
                }

                fn log10(self) -> Self {
                    self.log10()
                }

                fn sin(self) -> Self {
                    self.sin()
                }

                fn cos(self) -> Self {
                    self.cos()
                }

                fn tan(self) -> Self {
                    self.tan()
                }

                fn asin(self) -> Self {
                    self.asin()
                }

                fn acos(self) -> Self {
                    self.acos()
                }

                fn atan(self) -> Self {
                    self.atan()
                }

                fn sinh(self) -> Self {
                    self.sinh()
                }

                fn cosh(self) -> Self {
                    self.cosh()
                }

                fn tanh(self) -> Self {
                    self.tanh()
                }

                fn asinh(self) -> Self {
                    self.asinh()
                }

                fn acosh(self) -> Self {
                    self.acosh()
                }

                fn atanh(self) -> Self {
                    self.atanh()
                }
            }
        )*
    };
}

impl_real!(f32 f64);
