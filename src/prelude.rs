//! Import common traits.
//!
//! # Tutorial
//!
//! *WIP* 🚧

pub use traits::*;
pub use types::*;

pub use crate::{
    alloc_buffer,
    bus::Bus,
    frame::{
        splat,
        Arf,
        StSimd,
    },
    node::{
        heapnode,
        HeapNode,
        StackNode,
    },
    num::{
        one,
        one_half,
        tau,
        two,
        zero,
    },
    terp::{
        cub,
        lin,
        noi,
    },
};

pub mod types {
    pub use crate::{
        frame::{
            Mo,
            MoSimd,
            Qd,
            QdSimd,
            St,
            StSimd,
        },
        num::Fp,
    };
}

pub mod traits {
    pub use crate::{
        frame::Frame,
        node::Node,
        num::{
            Float,
            One,
            Real,
            Zero,
        },
        terp::{
            Cub,
            Lin,
            Noi,
        },
    };
}

pub mod dsp {
    pub use crate::{
        fbk::{
            Del,
            Fbk,
            Single,
        },
        filter::{
            butter_hipass,
            butter_lowpass,
            lpf,
            lpf2,
            lpf4,
            lpf8,
            resonator,
            Biquad,
            BiquadCoefs,
            DCBlock,
            Lpf,
            OnePole,
        },
        osc::{
            Sine,
            Wt,
        },
        util::Gain,
    };
}
