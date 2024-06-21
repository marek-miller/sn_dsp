//  lib.rs: entry
//    
//    Copyright 2023 ⧉⧉⧉
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

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
