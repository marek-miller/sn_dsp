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
};

use super::Frame;
use crate::num::{
    Float,
    Fp,
    Zero,
};

/// Array frame
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arf<T, const N: usize>([T; N]);

impl<T, const N: usize> Default for Arf<T, N>
where
    T: Float,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> From<[T; N]> for Arf<T, N>
where
    T: Float,
{
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> Zero for Arf<T, N>
where
    T: Float,
{
    fn zero() -> Self {
        Self([T::zero(); N])
    }
}

impl<T, const N: usize> Add for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] + rhs.0[i];
        }
        out
    }
}

impl<T, const N: usize> AddAssign for Arf<T, N>
where
    T: Float,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        for i in 0..N {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<T, const N: usize> Sub for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] - rhs.0[i];
        }
        out
    }
}

impl<T, const N: usize> SubAssign for Arf<T, N>
where
    T: Float,
{
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        for i in 0..N {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<T, const N: usize> Neg for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = -self.0[i];
        }
        out
    }
}

impl<T, const N: usize> Mul<T> for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] * rhs;
        }
        out
    }
}

impl<T, const N: usize> MulAssign<T> for Arf<T, N>
where
    T: Float,
{
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        for i in 0..N {
            self.0[i] *= rhs;
        }
    }
}

impl<T, const N: usize> Sum for Arf<T, N>
where
    T: Float,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, frame| acc + frame)
    }
}

impl<T, const N: usize> Frame for Arf<T, N>
where
    T: Float,
{
    type Sample = T;

    fn as_slice(&self) -> &[Self::Sample] {
        &self.0
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Sample] {
        &mut self.0
    }
}

impl<T> Arf<T, 2>
where
    T: Float,
{
    #[must_use]
    pub fn flip(self) -> Self {
        [self.0[1], self.0[0]].into()
    }

    /// `pos` is assumed to be within `[-1., 1.]`
    #[must_use]
    pub fn pan(
        self,
        pos: Fp,
    ) -> Self {
        let pan_l = ((1. - pos) * 0.5).sqrt().to_float();
        let pan_r = ((1. + pos) * 0.5).sqrt().to_float();
        [self.0[0] * pan_l, self.0[1] * pan_r].into()
    }
}
