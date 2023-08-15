use crate::num::{
    one,
    zero,
    Fp,
    One,
    Zero,
};

/// Polynomial curve
trait Poly {
    fn poly(t: Fp) -> Self;
}

enum Curve {
    Poly(Fp),
    Exp(Fp),
}

impl Default for Curve {
    fn default() -> Self {
        Curve::Poly(0.)
    }
}

struct Point<T>(T, T);

impl<T> Zero for Point<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self(zero(), zero())
    }
}

impl<T> One for Point<T>
where
    T: One,
{
    fn one() -> Self {
        Self(one(), one())
    }
}

impl<T> Default for Point<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::zero()
    }
}

struct Segment<T> {
    a:     Point<T>,
    b:     Point<T>,
    curve: Curve,
}

impl<T> Default for Segment<T>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            a:     Point::zero(),
            b:     Point::one(),
            curve: Curve::default(),
        }
    }
}
