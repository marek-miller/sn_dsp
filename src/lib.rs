//! DSP for sn_ 🪐
//!
//! Dynamic DSP system of nodes at variable control rate.

// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![feature(allocator_api)]
#![feature(new_uninit)]
#![feature(portable_simd)]

// pub mod prelude;

mod buffer;
pub use buffer::Buf;

mod control;
pub use control::{
    Control,
    Reset,
};

pub mod bus;
pub mod envelope;
pub mod feedback;
pub mod filter;
pub mod frame;
pub mod math;
pub mod node;
pub mod noise;
pub mod num;
pub mod oscillator;
pub mod util;
