//! DSP for sn_ 🪐
//!
//! Dynamic DSP system of nodes at variable control rate.
//!
//! See module: [prelude] for a tutorial.
//!
//! [predule]: crate::prelude

// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![feature(allocator_api)]
#![feature(new_uninit)]
#![feature(portable_simd)]

use std::alloc::Allocator;

pub mod bus;
pub mod envelope;
pub mod fbk;
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

#[must_use]
pub fn alloc_buffer_in<T, A>(
    size: usize,
    alloc: A,
) -> Box<[T], A>
where
    A: Allocator,
    T: Default,
{
    let mut b = Box::new_uninit_slice_in(size, alloc);
    for i in 0..size {
        b[i].write(T::default());
    }
    // SAFETY: All allocated memory has just been initialized
    unsafe { b.assume_init() }
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
