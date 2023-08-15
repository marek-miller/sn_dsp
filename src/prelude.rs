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
        Sdf,
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
        Bit,
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
            Qd,
            St,
        },
        num::Fp,
    };
}

pub mod traits {
    pub use crate::{
        envelope::Envelope,
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
        Control,
        Reset,
    };
}

pub mod dsp {
    pub use crate::{
        envelope::{
            Gate,
            Latch,
            Ramp,
            Trig,
        },
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
