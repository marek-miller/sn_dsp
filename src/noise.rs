use std::marker::PhantomData;

use rand::{
    Rng,
    SeedableRng,
};

use crate::{
    frame::Frame,
    node::Node,
    num::Float,
};

#[derive(Debug)]
pub struct Noise<T, R> {
    rng:     R,
    _marker: PhantomData<T>,
}

impl<T, R> Noise<T, R>
where
    R: Rng,
{
    pub fn with_rng(rng: R) -> Self {
        Self {
            rng,
            _marker: PhantomData,
        }
    }
}

impl<T, R> Noise<T, R>
where
    R: SeedableRng,
{
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng:     R::seed_from_u64(seed),
            _marker: PhantomData,
        }
    }
}

impl<T, R> From<u64> for Noise<T, R>
where
    R: SeedableRng,
{
    fn from(value: u64) -> Self {
        Self::with_seed(value)
    }
}

impl<T, R> Node for Noise<T, R>
where
    T: Frame,
    R: Rng,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        // TODO: Make it more efficient with Fill trait
        for frm in frames {
            for sample in frm.as_mut_slice() {
                *sample = self.rng.gen_range(-1. ..1.).to_float();
            }
        }
    }
}
