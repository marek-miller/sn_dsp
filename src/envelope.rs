//! Segmented envelope as a finite-state machine

use crate::{
    num::{
        one,
        zero,
        Bit,
        Float,
    },
    Reset,
};

mod geom;
pub use geom::{
    Curve,
    Line,
    Point,
    Segment,
};

pub trait Envelope {
    type Sample;

    fn tick(
        &mut self,
        gate: Gate,
    ) -> Self::Sample;

    fn advance_by(
        &mut self,
        steps: usize,
        gate: Gate,
    ) {
        for _ in 0..steps {
            let _ = self.tick(gate);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Gate {
    #[default]
    Closed,
    Open,
}

impl Gate {
    #[must_use]
    pub fn is_open(&self) -> bool {
        match self {
            Gate::Closed => false,
            Gate::Open => true,
        }
    }

    #[must_use]
    pub fn flip(self) -> Self {
        use Gate::{
            Closed,
            Open,
        };
        match self {
            Open => Closed,
            Closed => Open,
        }
    }
}

impl Reset for Gate {
    fn reset(&mut self) {
        *self = Gate::Closed;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Latch {
    #[default]
    Off,
    On,
}

impl Latch {
    #[must_use]
    pub fn is_on(&self) -> bool {
        match self {
            Latch::Off => false,
            Latch::On => true,
        }
    }
}

impl From<Latch> for Bit {
    fn from(value: Latch) -> Self {
        match value {
            Latch::Off => Bit::Zero,
            Latch::On => Bit::One,
        }
    }
}

impl Reset for Latch {
    fn reset(&mut self) {
        *self = Latch::Off;
    }
}

impl Envelope for Latch {
    type Sample = Bit;

    fn tick(
        &mut self,
        gate: Gate,
    ) -> Self::Sample {
        if gate.is_open() {
            *self = Latch::On;
        }
        (*self).into()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Trig {
    last_gate: Gate,
}

impl Trig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Reset for Trig {
    fn reset(&mut self) {
        self.last_gate.reset();
    }
}

impl Envelope for Trig {
    type Sample = Bit;

    fn tick(
        &mut self,
        gate: Gate,
    ) -> Self::Sample {
        let bit = if !self.last_gate.is_open() && gate.is_open() {
            one()
        } else {
            zero()
        };
        self.last_gate = gate;

        bit
    }
}

#[derive(Debug)]
pub struct Ramp<T> {
    pos:  Option<T>,
    rate: T,
}

impl<T> Ramp<T> {
    fn new(rate: T) -> Self {
        Self {
            pos: None,
            rate,
        }
    }
}

impl<T> From<T> for Ramp<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Reset for Ramp<T> {
    fn reset(&mut self) {
        self.pos = None;
    }
}

impl<T> Envelope for Ramp<T>
where
    T: Float,
{
    type Sample = T;

    fn tick(
        &mut self,
        gate: Gate,
    ) -> Self::Sample {
        match &mut self.pos {
            None => {
                if gate.is_open() {
                    self.pos = Some(zero());
                }
                zero()
            }
            Some(pos) => {
                if *pos >= one() {
                    one()
                } else {
                    *pos += self.rate;
                    *pos
                }
            }
        }
    }
}
