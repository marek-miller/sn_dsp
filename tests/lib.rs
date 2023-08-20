use sn_dsp::{
    bus::Bus,
    feedback::Del,
    frame::{
        splat,
        Mo,
        St,
    },
    node::{
        heapnode,
        Node,
        StackNode,
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
type Typ = St;

#[test]
fn check_dyn_chain_91() {
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

#[test]
fn check_dyn_chain_92() {
    let silence = Typ::zero();
    let impulse = splat(1.);

    let mut gain = 32.;

    let buf = Buf::alloc_new(2);
    let del1 = Del::new(buf);

    let mut chain = Bus::new();
    chain.node_push(del1);

    let node = StackNode::new(|frames| {
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

#[test]
fn check_node_tick_01() {
    let mut count = 0.;
    let mut node = StackNode::from(|frames: &mut [Mo]| {
        for frm in frames {
            count += 1.;
            *frm *= count;
        }
    });

    assert_eq!(node.tick(splat(1.)), splat(1.));
    assert_eq!(node.tick(splat(1.)), splat(2.));
    assert_eq!(node.tick(splat(1.)), splat(3.));
}
