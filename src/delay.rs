use std::mem;

use crate::{
    alloc_buffer,
    frame::Frame,
    node::Node,
    num::{
        Float,
        Fp,
    },
};

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

#[derive(Debug)]
pub struct Del<T> {
    pub feedback: Fp,
    buffer:       Box<[T]>,
    index:        usize,
}

impl<T> Del<T> {
    pub fn new(buffer: Box<[T]>) -> Self {
        Self {
            buffer,
            index: 0,
            feedback: 0.,
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
    // Alloce memory for a new buffer on the heap
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
            self.buffer[self.index] = *frm + y0 * self.feedback.to_float();
            self.index += 1;
            if self.index == self.buffer.len() {
                self.index = 0;
            }
            *frm = y0;
        }
    }
}
