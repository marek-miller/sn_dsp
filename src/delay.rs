use std::mem;

use crate::{
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
pub struct Delay<'a, T>
where
    T: Frame,
{
    pub feedback: Fp,
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
            feedback: 0.,
        }
    }
}

impl<'a, T> Node for Delay<'a, T>
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
