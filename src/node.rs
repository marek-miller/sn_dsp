#![allow(clippy::module_name_repetitions)]

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
    #[allow(clippy::type_complexity)]
    func: Box<dyn FnMut(&mut [T]) + 'a>,
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
    #[allow(clippy::type_complexity)]
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
