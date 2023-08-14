use sn_dsp::prelude::*;

#[test]
fn check_fbk_neutral_01() {
    let mut fbk = dsp::Fbk::new();

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
    let mut del = dsp::Del::new(buf);
    let node = heapnode(|frames| del.proc(frames));

    let mut fbk = dsp::Fbk::with_node(node);
    fbk.set_feedback(0.5);

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
    let mut del = dsp::Del::new(buf);
    let node = heapnode(|frames| del.proc(frames));

    let mut fbk = dsp::Fbk::with_node(node);
    fbk.set_feedback(0.5);

    let mut chain = Bus::new();
    chain.add_node(fbk);

    let sil = zero();
    let imp = splat(1.);

    let frames = &mut [imp, sil, sil, sil, sil, sil, sil, sil, sil];
    let expected = &[sil, sil, imp, sil, sil, imp * 0.5, sil, sil, imp * 0.25];
    chain.proc(frames);

    assert_eq!(frames, expected);
}
