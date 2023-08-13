use crate::{
    frame::Frame,
    node::Node,
    num::{
        one,
        zero,
    },
};

#[derive(Debug, Clone)]
pub struct OnePole<T>
where
    T: Frame,
{
    b0: T::Sample,
    a1: T::Sample,
    y1: T,
}

impl<T> Default for OnePole<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OnePole<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            b0: one(),
            a1: zero(),
            y1: zero(),
        }
    }
}

impl<T: Frame> Node for OnePole<T> {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = *frm * self.b0 - self.y1 * self.a1;
            self.y1 = y0;
            *frm = y0;
        }
    }
}
