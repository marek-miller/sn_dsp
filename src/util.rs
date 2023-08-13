use std::marker::PhantomData;

use crate::{
    frame::Frame,
    node::Node,
    num::one,
};

#[derive(Debug, Clone)]
pub struct Gain<T>
where
    T: Frame,
{
    pub gain: T::Sample,
    _marker:  PhantomData<T>,
}

impl<T> Default for Gain<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Gain<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            gain:    one(),
            _marker: PhantomData,
        }
    }
}

impl<T> Node for Gain<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm *= self.gain;
        }
    }
}
