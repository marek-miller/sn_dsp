use std::{
    alloc::{
        Allocator,
        Global,
    },
    ops::{
        Index,
        IndexMut,
    },
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
        Self((0..size).map(|_| T::default()).collect())
    }
}
