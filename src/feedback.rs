use std::{
    alloc::{
        Allocator,
        Global,
    },
    mem,
};

use crate::{
    bus::Bus,
    frame::Frame,
    node::Node,
    num::{
        zero,
        Float,
        Fp,
    },
    Buf,
};

#[derive(Debug)]
pub struct Single<T>(T);

impl<T> Single<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self(zero())
    }
}

impl<T> Default for Single<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Frame> Node for Single<T> {
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
pub struct Del<T, A = Global>
where
    A: Allocator,
{
    buffer: Buf<T, A>,
    index:  usize,
}

impl<T, A> Del<T, A>
where
    A: Allocator,
{
    #[must_use]
    pub fn new(buffer: Buf<T, A>) -> Self {
        Self {
            buffer,
            index: 0,
        }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.buffer.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.buffer.as_mut_slice()
    }

    #[must_use]
    pub fn into_buffer(self) -> Buf<T, A> {
        self.buffer
    }
}

impl<T, A> From<Buf<T, A>> for Del<T, A>
where
    A: Allocator,
{
    fn from(value: Buf<T, A>) -> Self {
        Self::new(value)
    }
}

impl<T> Del<T>
where
    T: Default,
{
    // Allocate memory for a new buffer on the heap
    #[must_use]
    pub fn alloc_new(size: usize) -> Self {
        Self::new(Buf::alloc_new(size))
    }
}

impl<T, A> Del<T, A>
where
    A: Allocator,
    T: Default,
{
    #[must_use]
    pub fn alloc_new_in(
        size: usize,
        alloc: A,
    ) -> Self {
        Self::new(Buf::alloc_new_in(size, alloc))
    }
}

impl<T, A> Node for Del<T, A>
where
    A: Allocator,
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
pub struct Fbk<'a, T, A = Global>
where
    A: Allocator,
{
    feedback:     Fp,
    ss_del_frame: T,
    bus:          Bus<'a, T, A>,
}

impl<'a, T> Fbk<'a, T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            feedback:     0.,
            ss_del_frame: zero(),
            bus:          Bus::new(),
        }
    }
}

impl<'a, T> Default for Fbk<'a, T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, A> Fbk<'a, T, A>
where
    A: Allocator,
    T: Frame,
{
    #[must_use]
    pub fn new_in(alloc: A) -> Self {
        Self {
            feedback:     0.,
            ss_del_frame: zero(),
            bus:          Bus::new_in(alloc),
        }
    }

    pub fn bus(&self) -> &Bus<'a, T, A> {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut Bus<'a, T, A> {
        &mut self.bus
    }

    pub fn into_bus(self) -> Bus<'a, T, A> {
        self.bus
    }

    pub fn feedback(&self) -> &Fp {
        &self.feedback
    }

    pub fn feedback_mut(&mut self) -> &mut Fp {
        &mut self.feedback
    }
}

impl<'a, T, A> Node for Fbk<'a, T, A>
where
    A: Allocator,
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = self
                .bus
                .tick(*frm + self.ss_del_frame * self.feedback.to_float());
            self.ss_del_frame = y0;
            *frm = y0;
        }
    }
}
