use std::{
    alloc::{
        Allocator,
        Global,
    },
    mem,
    ops::{
        Add,
        Index,
        IndexMut,
        Mul,
    },
    slice::{
        Iter,
        IterMut,
    },
    vec::IntoIter,
};

use crate::num::{
    one,
    zero,
    Float,
    Fp,
    One,
    Real,
    Zero,
};

#[derive(Debug)]
pub enum Curve {
    Poly(Fp),
    Exp(Fp),
}

impl Default for Curve {
    fn default() -> Self {
        Curve::Poly(0.)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point<T>(T, T);

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

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self(value.0, value.1)
    }
}

impl<T> Add for Point<T>
where
    T: Float,
{
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<T> Point<T>
where
    T: Real,
{
    /// Euclidean distance
    pub fn distance(
        &self,
        other: &Self,
    ) -> T {
        ((self.0 - other.0) * (self.0 - other.0)
            + (self.1 - other.1) * (self.1 - other.1))
            .sqrt()
    }

    /// Taxi-driver distance (l1-norm)
    pub fn taxi_distance(
        &self,
        other: &Self,
    ) -> T {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    /// Rotate counter-clockwise around origin (0.,0.)
    pub fn rotate(
        &mut self,
        angle: T,
    ) {
        let c = angle.cos();
        let s = angle.sin();
        let x = self.0 * c - self.1 * s;
        let y = self.0 * s + self.1 * c;
        self.0 = x;
        self.1 = y;
    }
}

#[derive(Debug)]
pub struct Segment<T> {
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

impl<T> Segment<T>
where
    T: PartialEq,
{
    pub fn with_points(
        a: Point<T>,
        b: Point<T>,
    ) -> Option<Self> {
        if a == b {
            None
        } else {
            Some(Self {
                a,
                b,
                curve: Curve::default(),
            })
        }
    }

    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    pub fn curve_mut(&mut self) -> &mut Curve {
        &mut self.curve
    }
}

// Collection of joint segments
#[derive(Debug)]
pub struct Line<T, A = Global>
where
    A: Allocator,
{
    seg: Vec<Segment<T>, A>,
}

impl<T, A> Line<T, A>
where
    A: Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            seg: Vec::new_in(alloc),
        }
    }

    pub fn with_capacity_in(
        capacity: usize,
        alloc: A,
    ) -> Self {
        Self {
            seg: Vec::with_capacity_in(capacity, alloc),
        }
    }

    pub fn as_slice(&self) -> &[Segment<T>] {
        &self.seg
    }

    pub fn as_mut_slice(&mut self) -> &[Segment<T>] {
        &mut self.seg
    }

    pub fn into_box(self) -> Box<[Segment<T>], A> {
        self.seg.into_boxed_slice()
    }

    pub fn iter(&self) -> Iter<'_, Segment<T>> {
        self.seg.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Segment<T>> {
        self.seg.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.seg.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seg.is_empty()
    }
}

impl<T> Line<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            seg: Vec::new()
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            seg: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Default for Line<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Line<T> {
    type IntoIter = IntoIter<Segment<T>>;
    type Item = Segment<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.seg.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Line<T> {
    type IntoIter = Iter<'a, Segment<T>>;
    type Item = &'a Segment<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Line<T> {
    type IntoIter = IterMut<'a, Segment<T>>;
    type Item = &'a mut Segment<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, A> Index<usize> for Line<T, A>
where
    A: Allocator,
{
    type Output = Segment<T>;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        &self.seg[index]
    }
}

impl<T, A> IndexMut<usize> for Line<T, A>
where
    A: Allocator,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.seg[index]
    }
}

impl<T, A> Line<T, A>
where
    A: Allocator,
{
    pub fn point_iter(&self) -> PointIter<'_, T, A> {
        PointIter::new(self)
    }
}

#[derive(Debug)]
pub struct PointIter<'a, T, A>
where
    A: Allocator,
{
    line:  &'a Line<T, A>,
    index: Option<usize>,
}

impl<'a, T, A> PointIter<'a, T, A>
where
    A: Allocator,
{
    pub fn new(line: &'a Line<T, A>) -> Self {
        Self {
            line,
            index: None,
        }
    }
}

impl<'a, T, A> Iterator for PointIter<'a, T, A>
where
    A: Allocator,
{
    type Item = &'a Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.is_empty() {
            return None;
        }
        match &mut self.index {
            None => {
                self.index = Some(0);
                Some(&self.line[0].a)
            }
            Some(i) => {
                if *i < self.line.len() {
                    let res = Some(&self.line[*i].b);
                    *i += 1;
                    res
                } else {
                    None
                }
            }
        }
    }
}

impl<T, A> Line<T, A>
where
    A: Allocator,
    T: Float,
{
    /// # Panics
    ///
    /// Panics if Line has no segments
    pub fn insert_point(
        &mut self,
        point: Point<T>,
    ) {
        assert!(!self.is_empty(), "Line has 0 segments");

        let mut idx = 0;
        for p in self.point_iter() {
            if p.0 > point.0 {
                break;
            }
            idx += 1;
        }

        if idx == 0 {
            let seg = Segment::with_points(point, self.seg[0].a)
                .expect("first envelope point should be different");
            self.seg.insert(0, seg);
        } else if idx == self.len() + 1 {
            let seg = Segment::with_points(self.seg[self.len() - 1].b, point)
                .expect("last envelope point should be different");
            self.seg.insert(self.len(), seg);
        } else {
            // break segment no.: idx -1 into two
            let seg1 =
                Segment::with_points(self.seg[idx - 1].a, point).unwrap();
            let seg2 =
                Segment::with_points(point, self.seg[idx - 1].b).unwrap();
            let _ = mem::replace(&mut self.seg[idx - 1], seg1);
            self.seg.insert(idx, seg2);
        }
    }
}
