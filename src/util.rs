use std::marker::PhantomData;

use crate::{
    frame::Frame,
    node::Node,
    num::{
        one,
        Float,
        Fp,
    },
};

#[derive(Debug, Clone)]
pub struct Util<T>
where
    T: Frame,
{
    pub gain: Fp,
    _marker:  PhantomData<T>,
}

impl<T> Util<T>
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

impl<T> Default for Util<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Node for Util<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm *= self.gain.to_float();
        }
    }
}
