//! DSP for sn_ 🪐
//!
//! Dynamic DSP system of nodes at variable control rate.
//!
//! See module: [prelude] for a tutorial.
//!
//! [predule]: crate::prelude

// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod delay;
pub mod filter;
pub mod frame;
pub mod node;
pub mod num;
pub mod osc;
pub mod terp;
pub mod util;

pub mod prelude;

#[must_use]
pub fn alloc_buffer<T: Default>(size: usize) -> Box<[T]> {
    (0..size).map(|_| T::default()).collect()
}

pub trait Reset {
    fn reset(&mut self);
}

pub trait Control {
    type Ctl<'a>
    where
        Self: 'a;

    fn control(
        &mut self,
        f: impl FnOnce(&mut Self::Ctl<'_>),
    );
}

impl<T> Reset for T
where
    T: Control,
    for<'a> <T as Control>::Ctl<'a>: Reset,
{
    fn reset(&mut self) {
        self.control(|ctl| ctl.reset());
    }
}
