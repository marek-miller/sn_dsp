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

#[allow(missing_debug_implementations)]
pub struct HeapNode<'a, T> {
    f: Box<dyn FnMut(&mut [T]) + 'a>,
}

impl<'a, T> Debug for HeapNode<'a, T> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("HeapNode")
            .field("func", &format_args!("Box<dyn FnMut>"))
            .finish()
    }
}

impl<'a, T> HeapNode<'a, T>
where
    T: Frame,
{
    /// Move closure `f` to the heap
    pub fn new(f: impl FnMut(&mut [T]) + 'a) -> Self {
        Self {
            f: Box::new(f)
        }
    }

    pub fn as_box(self) -> Box<dyn FnMut(&mut [T]) + 'a> {
        self.f
    }
}

/// Move closure `f` to the heap
pub fn heapnode<'a, T: Frame>(f: impl FnMut(&mut [T]) + 'a) -> HeapNode<'a, T> {
    HeapNode::new(f)
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

pub struct Bus<'a, T> {
    nodes: Vec<Box<dyn FnMut(&mut [T]) + 'a>>,
}

impl<'a, T> Debug for Bus<'a, T> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("Bus")
            .field("nodes", &format_args!("[Box<dyn FnMut>]"))
            .finish()
    }
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

    /// Move closure `f` to the heap
    pub fn push(
        &mut self,
        f: impl FnMut(&mut [T]) + 'a,
    ) {
        self.nodes.push(Box::new(f));
    }

    pub fn add_node(
        &mut self,
        node: HeapNode<'a, T>,
    ) where
        T: Frame,
    {
        self.nodes.push(node.as_box())
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
