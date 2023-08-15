use std::marker::PhantomData;

use rand::{
    Rng,
    SeedableRng,
};

use crate::{node::Node, frame::Frame};

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
    R: SeedableRng<Seed = u64>,
{
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng:     R::from_seed(seed),
            _marker: PhantomData,
        }
    }
}

impl<T, R> From<u64> for Noise<T, R>
where
    R: SeedableRng<Seed = u64>,
{
    fn from(value: u64) -> Self {
        Self::with_seed(value)
    }
}


impl<T,R> Node for Noise<T, R>
where T: Frame, R: Rng {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        todo!()
    }
}