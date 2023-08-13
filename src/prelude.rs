pub use traits::*;
pub use types::*;

pub use crate::{
    alloc_buffer,
    frame::{
        splat,
        Arf,
    },
    node::{
        Bus,
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
            Qd,
            St,
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
        delay::{
            Delay,
            SingleSample,
        },
        filter::OnePole,
        osc::{
            Sine,
            Wt,
        },
        util::Gain,
    };
}
