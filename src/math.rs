use std::ops::{
    Add,
    Mul,
};

use crate::num::Float;

pub mod geom;

/// Generic trait for non-interpolating values
pub trait Noi<T>
where
    T: Float,
{
    #[must_use]
    fn noi(
        self,
        other: Self,
        t: T,
    ) -> Self;
}

impl<U, T> Noi<T> for U
where
    T: Float,
{
    fn noi(
        self,
        other: Self,
        t: T,
    ) -> Self {
        if t < T::from_float(0.5) {
            self
        } else {
            other
        }
    }
}

/// Generic linear interpolation trait.
pub trait Lin<T>
where
    T: Float,
{
    #[must_use]
    fn lin(
        self,
        other: Self,
        t: T,
    ) -> Self;
}

impl<U, T> Lin<T> for U
where
    U: Add<Output = U> + Mul<T, Output = U>,
    T: Float,
{
    #[inline]
    fn lin(
        self,
        other: U,
        t: T,
    ) -> U {
        self * (T::one() - t) + other * t
    }
}

/// Generic cubic interpolation trait.
pub trait Cub<T>
where
    T: Float,
{
    #[must_use]
    fn cub(
        self,
        other: Self,
        prev: Self,
        next: Self,
        t: T,
    ) -> Self;
}

impl<U, T> Cub<T> for U
where
    U: Add<Output = U> + Mul<T, Output = U> + Copy,
    T: Float,
{
    #[inline]
    fn cub(
        self,
        other: Self,
        prev: Self,
        next: Self,
        t: T,
    ) -> Self {
        (((prev * T::from_float(-1.)
            + self * T::from_float(3.)
            + other * T::from_float(-3.)
            + next)
            * t
            + prev * T::from_float(2.)
            + self * T::from_float(-5.)
            + other * T::from_float(4.)
            + next * T::from_float(-1.))
            * t
            + prev * T::from_float(-1.)
            + other * T::from_float(1.))
            * (t * T::from_float(0.5))
            + self
    }
}

/// No interpolation. If `t < 0.5`, chose `a`; otherwise choose `b`.
pub fn noi<U, T>(
    a: U,
    b: U,
    t: T,
) -> U
where
    U: Noi<T>,
    T: Float,
{
    Noi::noi(a, b, t)
}

/// Linear interpolation of floating-point numbers.
#[inline]
pub fn lin<U, T>(
    a: U,
    b: U,
    t: T,
) -> U
where
    U: Lin<T>,
    T: Float,
{
    Lin::lin(a, b, t)
}

/// Catmull-Rom cubic spline interpolation.
///
/// A form of cubic Hermite spline. Interpolates between
/// `a` (returns `a` when `t` = 0) and
/// `b` (returns `b` when `t` = 1) while using the previous (`a_prev`)
/// and next (`b_next`) points to define slopes at the endpoints.
/// The maximum overshoot is 1/8th of the range of the arguments.
///
/// # Examples
///
/// ```rust
/// # use sn_dsp::math::cub;
/// let (a_prev, a, b, b_next) = (0., 1., 2., 3.);
///
/// assert_eq!(cub(a_prev, a, b, b_next, 0.00), 1.00);
/// assert_eq!(cub(a_prev, a, b, b_next, 0.25), 1.25);
/// assert_eq!(cub(a_prev, a, b, b_next, 1.00), 2.00);
/// ```
pub fn cub<U, T>(
    a_prev: U,
    a: U,
    b: U,
    b_next: U,
    t: T,
) -> U
where
    U: Cub<T>,
    T: Float,
{
    Cub::cub(a, b, a_prev, b_next, t)
}
