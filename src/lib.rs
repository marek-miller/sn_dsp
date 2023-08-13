//! DSP for sn_

// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod delay;
pub mod filter;
pub mod frame;
pub mod interp;
pub mod node;
pub mod num;
pub mod osc;
pub mod util;

#[cfg(test)]
mod test;

#[must_use]
pub fn alloc_buffer<T: Default>(size: usize) -> Box<[T]> {
    (0..size).map(|_| T::default()).collect()
}
