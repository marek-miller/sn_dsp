use std::{
    iter::Sum,
    ops::{
        Add,
        AddAssign,
        Mul,
        MulAssign,
        Neg,
        Sub,
        SubAssign,
    },
};

use crate::num::{
    Float,
    Fp,
    Zero,
};

mod arf;
pub use arf::Arf;

mod sdf;
pub use sdf::Sdf;

pub type Mo = Arf<Fp, 1>;
pub type St = Arf<Fp, 2>;
pub type Qd = Arf<Fp, 4>;

pub type MoSimd = Sdf<Fp, 1>;
pub type StSimd = Sdf<Fp, 2>;
pub type QdSimd = Sdf<Fp, 4>;

pub trait Frame:
    Default
    + Copy
    + Clone
    + PartialEq
    + Add<Output = Self>
    + AddAssign<Self>
    + Neg<Output = Self>
    + Mul<Self::Sample, Output = Self>
    + MulAssign<Self::Sample>
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum
    + Zero
{
    type Sample: Float;

    fn as_slice(&self) -> &[Self::Sample];
    fn as_mut_slice(&mut self) -> &mut [Self::Sample];

    fn splat(value: Self::Sample) -> Self {
        let mut frm = Self::zero();
        for x in frm.as_mut_slice() {
            *x = value;
        }
        frm
    }
}

#[inline]
#[must_use]
pub fn splat<T: Frame>(value: T::Sample) -> T {
    T::splat(value)
}
