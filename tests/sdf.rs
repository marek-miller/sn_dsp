use sn_dsp::prelude::*;

// Frame type
type Typ = StSimd;

#[test]
fn check_dyn_chain_91() {
    let silence = Typ::zero();
    let impulse = splat(1.);

    let mut gain = 32.;

    let buf = alloc_buffer(2);
    let del1 = dsp::Del::new(buf);

    let mut chain = Bus::new();
    chain.node_push(del1);

    let node = heapnode(|frames| {
        for frm in frames {
            gain /= 2.;
            *frm *= gain.to_float();
        }
    });
    chain.node_push(node);

    let mut frames = [impulse, impulse, silence, silence, silence];
    let expected = [silence, silence, impulse * 4., impulse * 2., silence];

    chain.proc(&mut frames);

    assert_eq!(frames, expected);
    drop(chain);
    assert!((gain - 1.).abs() < Fp::EPSILON);
}
