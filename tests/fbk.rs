use sn_dsp::{
    alloc_buffer,
    bus::Bus,
    fbk::{
        Del,
        Fbk,
    },
    frame::{
        splat,
        Mo,
    },
    node::Node,
    num::zero,
};

#[test]
fn check_fbk_neutral_01() {
    let mut fbk = Fbk::new();

    let sil = zero::<Mo>();
    let imp = splat(1.);

    let frames = &mut [imp, sil, sil, imp, sil];
    let expected = &[imp, sil, sil, imp, sil];
    fbk.proc(frames);

    assert_eq!(frames, expected);
}

#[test]
fn check_fbk_del_01() {
    let buf = alloc_buffer::<Mo>(2);
    let del = Del::new(buf);

    let mut fbk = Fbk::new();
    *fbk.feedback_mut() = 0.5;
    fbk.bus_mut().node_push(del);

    let sil = zero();
    let imp = splat(1.);

    let frames = &mut [imp, sil, sil, sil, sil, sil, sil, sil, sil];
    let expected = &[sil, sil, imp, sil, sil, imp * 0.5, sil, sil, imp * 0.25];
    fbk.proc(frames);

    assert_eq!(frames, expected);
}

#[test]
fn check_fbk_del_02() {
    let buf = alloc_buffer::<Mo>(2);
    let del = Del::new(buf);

    let mut fbk = Fbk::new();
    *fbk.feedback_mut() = 0.5;

    fbk.bus_mut().node_push(del);

    let mut chain = Bus::new();
    chain.node_push(fbk);

    let sil = zero();
    let imp = splat(1.);

    let frames = &mut [imp, sil, sil, sil, sil, sil, sil, sil, sil];
    let expected = &[sil, sil, imp, sil, sil, imp * 0.5, sil, sil, imp * 0.25];
    chain.proc(frames);

    assert_eq!(frames, expected);
}
