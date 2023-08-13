use std::{
    iter::Sum,
    marker::PhantomData,
    mem,
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

pub trait Zero {
    fn zero() -> Self;
}

#[inline]
#[must_use]
pub fn zero<T: Zero>() -> T {
    T::zero()
}

impl Zero for f32 {
    fn zero() -> Self {
        0.
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.
    }
}

pub trait One {
    fn one() -> Self;
}

#[inline]
#[must_use]
pub fn one<T: One>() -> T {
    T::one()
}

impl One for f32 {
    fn one() -> Self {
        1.
    }
}

impl One for f64 {
    fn one() -> Self {
        1.
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

pub trait Frame:
    Default
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
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

    fn splat(val: Self::Sample) -> Self {
        let mut frm = Self::zero();
        for x in frm.as_mut_slice() {
            *x = val;
        }
        frm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Arf<T, const N: usize>([T; N])
where
    T: Float;

impl<T, const N: usize> Default for Arf<T, N>
where
    T: Float,
{
    fn default() -> Self {
        Self::zero()
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

impl<T, const N: usize> One for Arf<T, N>
where
    T: Float,
{
    fn one() -> Self {
        Self([T::one(); N])
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

#[cfg(feature = "f64")]
pub type Fp = f64;
#[cfg(not(feature = "f64"))]
pub type Fp = f32;

pub type Mo = Arf<Fp, 1>;
pub type St = Arf<Fp, 2>;
pub type Qd = Arf<Fp, 4>;

pub trait Node {
    type Frame: Frame;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    );
}

#[derive(Debug, Clone, Default)]
pub struct PhaseFlip<T> {
    _marker: PhantomData<T>,
}

impl<T> Node for PhaseFlip<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm = -*frm;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gain<T>
where
    T: Frame,
{
    pub gain: T::Sample,
    _marker:  PhantomData<T>,
}

impl<T> Default for Gain<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Gain<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            gain:    one(),
            _marker: PhantomData,
        }
    }
}

impl<T> Node for Gain<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm *= self.gain;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SingleSample<T>(T);

impl<T: Frame> Node for SingleSample<T> {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            mem::swap(&mut self.0, frm);
        }
    }
}

#[derive(Debug, Clone)]
pub struct OnePole<T>
where
    T: Frame,
{
    b0: T::Sample,
    a1: T::Sample,
    y1: T,
}

impl<T> Default for OnePole<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OnePole<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            b0: one(),
            a1: zero(),
            y1: zero(),
        }
    }
}

impl<T: Frame> Node for OnePole<T> {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = *frm * self.b0 - self.y1 * self.a1;
            self.y1 = y0;
            *frm = y0;
        }
    }
}

#[must_use]
pub fn alloc_buffer<T: Default>(size: usize) -> Box<[T]> {
    (0..size).map(|_| T::default()).collect()
}

#[derive(Debug)]
pub struct Delay<'a, T>
where
    T: Frame,
{
    pub feedback: T::Sample,
    buffer:       &'a mut [T],
    index:        usize,
}

impl<'a, T> Delay<'a, T>
where
    T: Frame,
{
    pub fn new(buffer: &'a mut [T]) -> Self {
        Self {
            buffer,
            index: 0,
            feedback: zero(),
        }
    }
}

impl<'a, T> Node for Delay<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = self.buffer[self.index];
            self.buffer[self.index] = *frm + y0 * self.feedback;
            self.index += 1;
            if self.index == self.buffer.len() {
                self.index = 0;
            }
            *frm = y0;
        }
    }
}

pub struct StackNode<T, F>
where
    T: Frame,
    F: FnMut(&mut [T]),
{
    func:    F,
    _marker: PhantomData<T>,
}

impl<T, F> StackNode<T, F>
where
    T: Frame,
    F: FnMut(&mut [T]),
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            _marker: PhantomData,
        }
    }
}

impl<T, F> Node for StackNode<T, F>
where
    T: Frame,
    F: FnMut(&mut [T]),
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        (self.func)(frames);
    }
}

pub struct HeapNode<'a, T>
where
    T: Frame,
{
    func: Box<dyn FnMut(&mut [T]) + 'a>,
}

impl<'a, T> HeapNode<'a, T>
where
    T: Frame,
{
    /// Moves `func` to the heap
    pub fn new(func: impl FnMut(&mut [T]) + 'a) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

impl<'a, T> Node for HeapNode<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        (self.func)(frames);
    }
}

pub struct Bus<'a, T> {
    nodes: Vec<Box<dyn FnMut(&mut [T]) + 'a>>,
}

impl<'a, T> Default for Bus<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Bus<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    /// Allocates memory on the heap
    pub fn push(
        &mut self,
        func: impl FnMut(&mut [T]) + 'a,
    ) {
        self.nodes.push(Box::new(func));
    }

    /// Allocates memory on the heap
    pub fn add_node(
        &mut self,
        node: impl Node<Frame = T> + 'a,
    ) {
        let mut node = node;
        self.push(move |x| node.proc(x));
    }
}

impl<'a, T> Node for Bus<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for func in &mut self.nodes {
            func(frames);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sine<T>
where
    T: Frame,
{
    pub phase: T::Sample,
    pub freq:  T::Sample,
}

impl<T> Sine<T>
where
    T: Frame,
{
    pub fn new(freq: T::Sample) -> Self {
        Self {
            phase: zero(),
            freq,
        }
    }
}

impl<T> Node for Sine<T>
where
    T: Frame,
    T::Sample: Real,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm = Self::Frame::splat((self.phase * tau()).sin());
            self.phase += self.freq;
            while self.phase >= one() {
                self.phase -= one();
            }
        }
    }
}

#[test]
fn check_dyn_chain_91() {
    let mut buf = alloc_buffer(2);
    let del1 = Delay::new(&mut buf);

    let mut gain = 32.;

    let mut chain = Bus::new();
    chain.add_node(del1);

    chain.push(|frames| {
        for frm in frames {
            gain /= 2.;
            *frm *= gain;
        }
    });

    let silence = St::zero();
    let impulse = St::splat(1.);

    let mut frames = [impulse, impulse, silence, silence, silence];
    let expected = [silence, silence, impulse * 4., impulse * 2., silence];

    chain.proc(&mut frames);

    assert_eq!(frames, expected);
    drop(chain);
    assert!((gain - 1.).abs() < Fp::EPSILON);
}
