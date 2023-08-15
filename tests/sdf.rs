use sn_dsp::{
    bus::Bus,
    fbk::Del,
    frame::{
        splat,
        Sdf,
    },
    node::{
        heapnode,
        Node,
    },
    num::{
        Float,
        Fp,
        Real,
        Zero,
    },
    Buf,
};

// Frame type
type Typ = Sdf<Fp, 2>;

#[test]
fn check_dyn_chain_simd_91() {
    let silence = Typ::zero();
    let impulse = splat(1.);

    let mut gain = 32.;

    let buf = Buf::alloc_new(2);
    let del1 = Del::new(buf);

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
