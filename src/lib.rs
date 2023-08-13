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
