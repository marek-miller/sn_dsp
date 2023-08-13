use crate::{
    alloc_buffer,
    delay::Delay,
    frame::{
        Frame,
        St,
    },
    node::{
        Bus,
        Node,
    },
    num::{
        zero,
        Fp,
    },
};

#[test]
fn check_dyn_chain_91() {
    let mut buf = alloc_buffer(2);
    let del1 = Delay::new(&mut buf);

    let mut gain = 32.;

    let mut chain = Bus::new();
    chain.add_node(del1);

    chain.push(|frames| {
        for frm in frames {
            gain /= 2.;
            *frm *= gain;
        }
    });

    let silence = zero();
    let impulse = St::splat(1.);

    let mut frames = [impulse, impulse, silence, silence, silence];
    let expected = [silence, silence, impulse * 4., impulse * 2., silence];

    chain.proc(&mut frames);

    assert_eq!(frames, expected);
    drop(chain);
    assert!((gain - 1.).abs() < Fp::EPSILON);
}
