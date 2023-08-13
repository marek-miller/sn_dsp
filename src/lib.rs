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
    + Sum
    + Zero
{
    type Float: Float;

    fn as_slice(&self) -> &[Self::Float];
    fn as_mut_slice(&mut self) -> &mut [Self::Float];

    fn splat(val: Self::Float) -> Self {
        let mut frm = Self::zero();
        for x in frm.as_mut_slice() {
            *x = val;
        }
        frm
    }
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

impl<T, const N: usize> Sum for ArrayFrame<T, N>
where
    T: Float,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, frame| acc + frame)
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

pub type Flt = f64;
pub type Frm<const N: usize> = ArrayFrame<Flt, N>;
pub type Mo = Frm<1>;
pub type St = Frm<2>;
pub type Qd = Frm<4>;

pub trait Node {
    type Frame: Frame;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame;
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
    pub gain: T::Float,
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

#[derive(Debug, Clone)]
pub struct OnePole<T>
where
    T: Frame,
{
    b0: T::Float,
    a1: T::Float,
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

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        let y0 = frm * self.b0 - self.y1 * self.a1;
        self.y1 = y0;
        y0
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
    pub feedback: T::Float,
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

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        let out = self.buffer[self.index];
        self.buffer[self.index] = frm + out * self.feedback;

        self.index += 1;
        if self.index == self.buffer.len() {
            self.index = 0;
        }

        out
    }
}

pub struct StackNode<T, F>
where
    T: Frame,
    F: FnMut(T) -> T,
{
    func:    F,
    _marker: PhantomData<T>,
}

impl<T, F> StackNode<T, F>
where
    T: Frame,
    F: FnMut(T) -> T,
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
    F: FnMut(T) -> T,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        (self.func)(frm)
    }
}

pub struct HeapNode<'a, T>
where
    T: Frame,
{
    func: Box<dyn FnMut(T) -> T + 'a>,
}

impl<'a, T> HeapNode<'a, T>
where
    T: Frame,
{
    /// Moves `func` to the heap
    pub fn new(func: impl FnMut(T) -> T + 'a) -> Self {
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

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        (self.func)(frm)
    }
}

pub struct Bus<'a, T> {
    nodes: Vec<Box<dyn FnMut(T) -> T + 'a>>,
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
        func: impl FnMut(T) -> T + 'a,
    ) {
        self.nodes.push(Box::new(func));
    }

    /// Allocates memory on the heap
    pub fn add_node(
        &mut self,
        node: impl Node<Frame = T> + 'a,
    ) {
        let mut node = node;
        self.push(move |x| node.tick(x));
    }
}

impl<'a, T> Node for Bus<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        self.nodes.iter_mut().fold(frm, |acc, func| func(acc))
    }
}

pub struct Mix<'a, T> {
    nodes: Vec<Box<dyn FnMut(T) -> T + 'a>>,
}

impl<'a, T> Default for Mix<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Mix<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    /// Allocates memory on the heap
    pub fn push(
        &mut self,
        func: impl FnMut(T) -> T + 'a,
    ) {
        self.nodes.push(Box::new(func));
    }

    /// Allocates memory on the heap
    pub fn add_node(
        &mut self,
        node: impl Node<Frame = T> + 'a,
    ) {
        let mut node = node;
        self.push(move |x| node.tick(x));
    }
}

impl<'a, T> Node for Mix<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn tick(
        &mut self,
        frm: Self::Frame,
    ) -> Self::Frame {
        self.nodes.iter_mut().map(|func| func(frm)).sum()
    }
}

#[test]
fn check_dyn_chain_91() {
    let mut buf = alloc_buffer(2);
    let del1 = Delay::new(&mut buf);

    let mut gain = 32.;

    let mut chain = Bus::new();
    chain.add_node(del1);

    chain.push(|x| {
        gain /= 2.;
        x * gain
    });

    let silence = St::zero();
    let impulse = St::splat(1.);

    assert_eq!(chain.tick(impulse), silence);
    assert_eq!(chain.tick(impulse), silence);
    assert_eq!(chain.tick(silence), impulse * 4.);
    assert_eq!(chain.tick(silence), impulse * 2.);
    assert_eq!(chain.tick(silence), silence);

    drop(chain);
    assert!((gain - 1.).abs() < Flt::EPSILON);
}

#[test]
fn check_dyn_mix_91() {
    let mut buf = alloc_buffer(2);
    let del1 = Delay::new(&mut buf);

    let mut gain = 32.;
    let mut mix = Mix::new();
    mix.add_node(del1);

    mix.push(|x| {
        gain /= 2.;
        x * gain
    });

    let silence = St::zero();
    let impulse = St::splat(1.);

    assert_eq!(mix.tick(impulse), impulse * 16.);
    assert_eq!(mix.tick(impulse), impulse * 8.);
    assert_eq!(mix.tick(impulse), impulse * (4. + 1.));
    assert_eq!(mix.tick(impulse), impulse * (2. + 1.));
    assert_eq!(mix.tick(silence), impulse);
    assert_eq!(mix.tick(silence), impulse);
    assert_eq!(mix.tick(silence), silence);

    drop(mix);
    assert!((gain - 0.25).abs() < Flt::EPSILON);
}
