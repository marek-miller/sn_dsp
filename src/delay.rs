use std::mem;

use crate::{
    alloc_buffer,
    frame::Frame,
    node::{
        HeapNode,
        Node,
    },
    num::{
        zero,
        Float,
        Fp,
    },
};

#[derive(Debug, Default)]
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

#[derive(Debug)]
pub struct Del<T> {
    buffer: Box<[T]>,
    index:  usize,
}

impl<T> Del<T> {
    pub fn new(buffer: Box<[T]>) -> Self {
        Self {
            buffer,
            index: 0,
        }
    }

    pub fn buffer(&self) -> &[T] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [T] {
        &mut self.buffer
    }

    pub fn into_buffer(self) -> Box<[T]> {
        self.buffer
    }
}

impl<T> Del<T>
where
    T: Default,
{
    // Allocate memory for a new buffer on the heap
    pub fn alloc_new(size: usize) -> Self {
        Self::new(alloc_buffer(size))
    }
}

impl<T> Node for Del<T>
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
            self.buffer[self.index] = *frm;
            self.index += 1;
            if self.index == self.buffer.len() {
                self.index = 0;
            }
            *frm = y0;
        }
    }
}
#[derive(Debug)]
pub struct Fbk<'a, T> {
    feedback:     Fp,
    ss_del_frame: T,
    node:         Option<HeapNode<'a, T>>,
}

impl<'a, T> Fbk<'a, T>
where
    T: Frame,
{
    pub fn new() -> Self {
        Self {
            feedback:     0.,
            ss_del_frame: zero(),
            node:         None,
        }
    }

    pub fn with_node(node: HeapNode<'a, T>) -> Self {
        let mut fbk = Self::new();
        fbk.node = Some(node);
        fbk
    }

    pub fn with_feedback(value: Fp) -> Self {
        let mut fbk = Self::new();
        fbk.feedback = value;
        fbk
    }

    pub fn feedback(&self) -> Fp {
        self.feedback
    }

    pub fn set_feedback(
        &mut self,
        value: Fp,
    ) {
        self.feedback = value;
    }

    pub fn swap_node(
        &mut self,
        node: HeapNode<'a, T>,
    ) -> Option<HeapNode<'a, T>> {
        mem::replace(&mut self.node, Some(node))
    }
}

impl<'a, T> Node for Fbk<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y = &mut [*frm + self.ss_del_frame * self.feedback.to_float()];
            if let Some(node) = &mut self.node {
                node.proc(y);
            }
            self.ss_del_frame = y[0];
            *frm = y[0];
        }
    }
}
