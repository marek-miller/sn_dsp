//! Segmented envelope as a finite-state machine

use crate::num::{
    one,
    zero,
    Float,
};

mod geom;

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
pub enum Trig {
    #[default]
    Off,
    On,
}

impl Trig {
    #[must_use]
    pub fn is_on(&self) -> bool {
        match self {
            Trig::Off => false,
            Trig::On => true,
        }
    }
}

impl Envelope for Trig {
    type Sample = Self;

    fn tick(
        &mut self,
        gate: Gate,
    ) -> Self::Sample {
        if gate.is_open() && !self.is_on() {
            *self = Trig::On;
        } else {
            *self = Trig::Off;
        }
        *self
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
