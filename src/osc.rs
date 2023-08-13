use std::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    frame::Frame,
    node::Node,
    num::{
        tau,
        Float,
        Fp,
        Real,
    },
    terp::lin,
};

#[derive(Debug, Clone)]
pub struct Sine<T> {
    pub phase: Fp,
    pub freq:  Fp,
    _marker:   PhantomData<T>,
}

impl<T> Sine<T> {
    #[must_use]
    pub fn new(freq: Fp) -> Self {
        Self {
            phase: 0.,
            freq,
            _marker: PhantomData,
        }
    }
}

impl<T> Node for Sine<T>
where
    T: Frame,
    T::Sample: Real,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            *frm = Self::Frame::splat(
                (T::Sample::from_float(self.phase) * tau()).sin(),
            );
            self.phase += self.freq;
            while self.phase >= 1. {
                self.phase -= 1.;
            }
        }
    }
}

/// Wavetable oscillator with linear interpolation
#[derive(Debug)]
pub struct Wt<'a, T> {
    pub phase: Fp,
    pub freq:  Fp,
    wt:        &'a [T],
}

impl<'a, T> Wt<'a, T> {
    pub fn new(
        wt: &'a [T],
        freq: Fp,
    ) -> Self {
        Self {
            phase: 0.,
            freq,
            wt,
        }
    }
}
impl<'a, T> Node for Wt<'a, T>
where
    T: Frame,
{
    type Frame = T;

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let len = self.wt.len();
            let idx_fp = len as Fp * self.phase;
            let idx = idx_fp.floor();
            let t = idx_fp - idx;
            let idx = if idx as usize >= len { 0 } else { idx as usize };
            let idx_next = if idx == len - 1 { 0 } else { idx + 1 };

            *frm = lin(self.wt[idx], self.wt[idx_next], t.to_float());
            self.phase += self.freq;
            while self.phase >= 1. {
                self.phase -= 1.;
            }
        }
    }
}
