use std::{
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

impl Float for f32 {
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

pub trait Frame:
    Default
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign<Self>
    + Neg<Output = Self>
    + Mul<Self::Float, Output = Self>
    + MulAssign<Self::Float>
    + Sub<Output = Self>
    + SubAssign<Self>
    + Zero
{
    type Float: Float;

    fn as_slice(&self) -> &[Self::Float];
    fn as_mut_slice(&mut self) -> &mut [Self::Float];
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ArrayFrame<T, const N: usize>([T; N])
where
    T: Float;

impl<T, const N: usize> Default for ArrayFrame<T, N>
where
    T: Float,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> Zero for ArrayFrame<T, N>
where
    T: Float,
{
    fn zero() -> Self {
        Self([T::zero(); N])
    }
}

impl<T, const N: usize> One for ArrayFrame<T, N>
where
    T: Float,
{
    fn one() -> Self {
        Self([T::one(); N])
    }
}

impl<T, const N: usize> Add for ArrayFrame<T, N>
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

impl<T, const N: usize> AddAssign for ArrayFrame<T, N>
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

impl<T, const N: usize> Sub for ArrayFrame<T, N>
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

impl<T, const N: usize> SubAssign for ArrayFrame<T, N>
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

impl<T, const N: usize> Neg for ArrayFrame<T, N>
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

impl<T, const N: usize> Mul<T> for ArrayFrame<T, N>
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

impl<T, const N: usize> MulAssign<T> for ArrayFrame<T, N>
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

impl<T, const N: usize> Frame for ArrayFrame<T, N>
where
    T: Float,
{
    type Float = T;

    fn as_slice(&self) -> &[Self::Float] {
        &self.0
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Float] {
        &mut self.0
    }
}

pub type Frm32<const N: usize> = ArrayFrame<f32, N>;
pub type Frm64<const N: usize> = ArrayFrame<f64, N>;

pub type Mono<T> = ArrayFrame<T, 1>;
pub type Stereo<T> = ArrayFrame<T, 2>;
pub type Quad<T> = ArrayFrame<T, 4>;

pub type Mo = Mono<f64>;
pub type St = Stereo<f64>;
pub type Qd = Quad<f64>;

pub trait Node {
    type Frame: Frame;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame;

    fn chain<K>(
        self,
        other: K,
    ) -> Chain<Self, K>
    where
        Self: Sized,
        K: Node<Frame = Self::Frame>,
    {
        Chain(self, other)
    }

    fn mix<K>(
        self,
        other: K,
    ) -> Mix<Self, K>
    where
        Self: Sized,
        K: Node<Frame = Self::Frame>,
    {
        Mix(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct PhaseFlip<T> {
    _marker: PhantomData<T>,
}

impl<T> Node for PhaseFlip<T>
where
    T: Frame,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        -frm
    }
}

#[derive(Debug, Clone)]
pub struct Gain<T>
where
    T: Frame,
{
    gain:    T::Float,
    _marker: PhantomData<T>,
}

impl<T> Node for Gain<T>
where
    T: Frame,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        frm * self.gain
    }
}

#[derive(Debug, Clone, Default)]
pub struct SingleSample<T>(T);

impl<T: Frame> Node for SingleSample<T> {
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        mem::replace(&mut self.0, frm)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OnePole<T>
where
    T: Frame,
{
    b0: T::Float,
    a1: T::Float,
    y1: T,
}

impl<T: Frame> Node for OnePole<T> {
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        let y0 = frm * self.b0 - self.y1 * self.a1;
        self.y1 = y0;
        y0
    }
}

#[derive(Debug, Clone)]
pub struct Chain<T, K>(T, K);

impl<T, K> Node for Chain<T, K>
where
    T: Node,
    K: Node<Frame = T::Frame>,
{
    type Frame = T::Frame;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        self.1.tick(self.0.tick(frm))
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
    buffer: &'a mut [T],
    index:  usize,
    fbk:    T::Float,
}

impl<'a, T> Delay<'a, T>
where
    T: Frame,
{
    pub fn new(buffer: &'a mut [T]) -> Self {
        Self {
            buffer,
            index: 0,
            fbk: zero(),
        }
    }

    pub fn feedback(
        &mut self,
        value: T::Float,
    ) {
        self.fbk = value;
    }
}

impl<'a, T> Node for Delay<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        let out = self.buffer[self.index];
        self.buffer[self.index] = frm + out * self.fbk;

        self.index += 1;
        if self.index == self.buffer.len() {
            self.index = 0;
        }

        out
    }
}

#[derive(Debug)]
pub struct Mix<T, K>(T, K);

impl<T, K> Node for Mix<T, K>
where
    T: Node,
    K: Node<Frame = T::Frame>,
{
    type Frame = T::Frame;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        self.0.tick(frm) + self.1.tick(frm)
    }
}
