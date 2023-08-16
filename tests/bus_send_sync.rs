use std::thread;

use sn_dsp::{
    bus::Bus,
    feedback::Del,
    frame::Mo,
    node::Node,
    num::{
        zero,
        Zero,
    },
};

#[test]
fn send_bus() {
    let mut bus = Bus::new();

    let del = Del::alloc_new(123);
    bus.push(Box::new(del));

    thread::scope(|s| {
        s.spawn(|| {
            let mut frames = vec![Mo::zero(); 8];
            bus.proc(&mut frames);
        });
    });

    bus.proc(&mut [zero()]);
}
