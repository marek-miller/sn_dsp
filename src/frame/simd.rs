use std::{
    fmt::Debug,
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
    simd::{
        LaneCount,
        Simd,
        SimdElement,
        SupportedLaneCount,
    },
};

use super::{
    Arf,
    Frame,
};
use crate::num::{
    Float,
    Zero,
};

/// Frame of samples implemented as SIMD vector.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sdf<T, const N: usize>(Simd<T, N>)
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement;

impl<T, const N: usize> From<[T; N]> for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    fn from(value: [T; N]) -> Self {
        Self(value.into())
    }
}

impl<T, const N: usize> From<Sdf<T, N>> for [T; N]
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    fn from(value: Sdf<T, N>) -> Self {
        value.0.to_array()
    }
}

impl<T, const N: usize> From<Simd<T, N>> for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    fn from(value: Simd<T, N>) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> From<Arf<T, N>> for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    fn from(value: Arf<T, N>) -> Self {
        let arr: [T; N] = value.into();
        Self(arr.into())
    }
}

impl<T, const N: usize> From<Sdf<T, N>> for Arf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
{
    fn from(value: Sdf<T, N>) -> Self {
        let arr: [T; N] = value.into();
        arr.into()
    }
}

impl<T, const N: usize> Zero for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement + Float,
{
    fn zero() -> Self {
        Self(Simd::splat(T::zero()))
    }
}

impl<T, const N: usize> Default for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement + Float,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> Add for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: Add<Output = Simd<T, N>>,
{
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T, const N: usize> AddAssign for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: AddAssign,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0 += rhs.0;
    }
}

impl<T, const N: usize> Sub for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: Sub<Output = Simd<T, N>>,
{
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T, const N: usize> SubAssign for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: SubAssign,
{
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0 -= rhs.0;
    }
}

impl<T, const N: usize> Mul<T> for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement + Float,
{
    type Output = Self;

    fn mul(
        mut self,
        rhs: T,
    ) -> Self::Output {
        self.0.as_mut_array().iter_mut().for_each(|x| {
            *x *= rhs;
        });
        self
    }
}

impl<T, const N: usize> MulAssign<T> for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement + Float,
{
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        self.0.as_mut_array().iter_mut().for_each(|x| {
            *x = *x * rhs;
        });
    }
}

impl<T, const N: usize> Neg for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: Neg<Output = Simd<T, N>>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T, const N: usize> Sum for Sdf<T, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SimdElement,
    Simd<T, N>: Sum,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|frm| frm.0).sum())
    }
}

impl<T, const N: usize> Frame for Sdf<T, N>
where
    T: SimdElement + Float,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: Add<Output = Simd<T, N>>
        + AddAssign
        + Sub<Output = Simd<T, N>>
        + SubAssign
        + Neg<Output = Simd<T, N>>
        + Sum,
{
    type Sample = T;

    fn as_slice(&self) -> &[Self::Sample] {
        self.0.as_array().as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Sample] {
        self.0.as_mut_array().as_mut_slice()
    }

    fn splat(value: Self::Sample) -> Self {
        Self(Simd::splat(value))
    }
}
