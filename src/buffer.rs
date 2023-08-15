use std::{
    alloc::{
        Allocator,
        Global,
    },
    ops::{
        Index,
        IndexMut,
    },
    slice::{
        Iter,
        IterMut,
    },
};

use crate::num::{
    zero,
    Zero,
};

#[derive(Debug)]
pub struct Buf<T, A = Global>(Box<[T], A>)
where
    A: Allocator;

impl<T, A> Buf<T, A>
where
    A: Allocator,
{
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_box(self) -> Box<[T], A> {
        self.0
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.0.iter_mut()
    }
}

impl<T, A> Index<usize> for Buf<T, A>
where
    A: Allocator,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, A> IndexMut<usize> for Buf<T, A>
where
    A: Allocator,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a, T, A> IntoIterator for &'a Buf<T, A>
where
    A: Allocator,
{
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T, A> IntoIterator for &'a mut Buf<T, A>
where
    A: Allocator,
{
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T, A> Buf<T, A>
where
    A: Allocator,
    T: Default,
{
    #[must_use]
    pub fn alloc_new_in(
        size: usize,
        alloc: A,
    ) -> Self
    where
        A: Allocator,
        T: Default,
    {
        let mut b = Box::new_uninit_slice_in(size, alloc);
        for i in 0..size {
            b[i].write(T::default());
        }
        // SAFETY: All allocated memory has just been initialized
        Self(unsafe { b.assume_init() })
    }

    pub fn clear(&mut self) {
        for x in self {
            *x = T::default()
        }
    }
}

impl<T, A> Buf<T, A>
where
    A: Allocator,
    T: Zero,
{
    pub fn zero(&mut self) {
        for x in self {
            *x = zero()
        }
    }
}

impl<T, A> Buf<T, A>
where
    A: Allocator,
    T: Copy + Default,
{
    pub fn splat_in(
        size: usize,
        value: T,
        alloc: A,
    ) -> Self {
        let mut buf = Self::alloc_new_in(size, alloc);
        for x in &mut buf {
            *x = value;
        }
        buf
    }
}

impl<T, A> From<Box<[T], A>> for Buf<T, A>
where
    A: Allocator,
{
    fn from(value: Box<[T], A>) -> Self {
        Self(value)
    }
}

impl<T> Buf<T>
where
    T: Default,
{
    #[must_use]
    pub fn alloc_new(size: usize) -> Self {
        Self::alloc_new_in(size, Global)
    }
}

impl<T> Buf<T>
where
    T: Default + Copy,
{
    #[must_use]
    pub fn splat(
        size: usize,
        value: T,
    ) -> Self {
        Self::splat_in(size, value, Global)
    }
}
