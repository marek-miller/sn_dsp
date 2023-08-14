#![allow(clippy::module_name_repetitions)]
#![allow(clippy::type_complexity)]

use std::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::frame::Frame;

pub trait Node {
    type Frame: Frame;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    );
}

#[derive(Debug)]
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

impl<T, F> From<F> for StackNode<T, F>
where
    T: Frame,
    F: FnMut(&mut [T]),
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

pub struct HeapNode<'a, T> {
    f: Box<dyn FnMut(&mut [T]) + 'a>,
}

impl<'a, T> Debug for HeapNode<'a, T> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("HeapNode")
            .field("func", &format_args!("Box<dyn FnMut(..)>"))
            .finish()
    }
}

impl<'a, T> HeapNode<'a, T>
where
    T: Frame,
{
    #[must_use]
    pub fn new(f: Box<dyn FnMut(&mut [T]) + 'a>) -> Self {
        Self {
            f,
        }
    }

    #[must_use]
    pub fn as_box(&self) -> &Box<dyn FnMut(&mut [T]) + 'a> {
        &self.f
    }

    pub fn as_box_mut(&mut self) -> &mut Box<dyn FnMut(&mut [T]) + 'a> {
        &mut self.f
    }

    #[must_use]
    pub fn into_box(self) -> Box<dyn FnMut(&mut [T]) + 'a> {
        self.f
    }
}

impl<'a, T> From<Box<dyn FnMut(&mut [T]) + 'a>> for HeapNode<'a, T>
where
    T: Frame,
{
    fn from(value: Box<dyn FnMut(&mut [T]) + 'a>) -> Self {
        Self::new(value)
    }
}

/// Move closure `f` to the heap
pub fn heapnode<'a, T: Frame>(f: impl FnMut(&mut [T]) + 'a) -> HeapNode<'a, T> {
    HeapNode::new(Box::new(f))
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
        (self.f)(frames);
    }
}
