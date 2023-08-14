use sn_dsp::{
    Control,
    Reset,
};

struct Mock {
    feedback: u16,
}

struct MockCtl<'a> {
    a: &'a mut u16,
}

impl<'a> Reset for MockCtl<'a> {
    fn reset(&mut self) {
        *self.a = 77;
    }
}

impl Control for Mock {
    type Ctl<'a> = MockCtl<'a> where Self: 'a;

    fn control(
        &mut self,
        f: impl FnOnce(&mut Self::Ctl<'_>),
    ) {
        let mut ctl = MockCtl {
            a: &mut self.feedback,
        };
        f(&mut ctl);
        self.feedback = *ctl.a + 1;
    }
}

#[test]
fn check_control_01() {
    let mut mock = Mock {
        feedback: 9
    };
    assert_eq!(mock.feedback, 9);

    mock.control(|fbk| *fbk.a = 11);

    assert_eq!(mock.feedback, 12);
}

#[test]
fn check_reset_01() {
    let mut mock = Mock {
        feedback: 9
    };
    assert_eq!(mock.feedback, 9);

    mock.control(|fbk| fbk.reset());

    assert_eq!(mock.feedback, 78);
}

#[test]
fn check_reset_impl() {
    let mut mock = Mock {
        feedback: 9
    };
    assert_eq!(mock.feedback, 9);

    mock.reset();

    assert_eq!(mock.feedback, 78);
}

struct Mock2 {
    feedback: u16,
}

impl Control for Mock2 {
    type Ctl<'a> = (&'a mut u16,);

    fn control(
        &mut self,
        f: impl FnOnce(&mut Self::Ctl<'_>),
    ) {
        f(&mut (&mut self.feedback,));
    }
}

#[test]
fn check_control_mock2_01() {
    let mock2 = Mock2 {
        feedback: 9
    };
    assert_eq!(mock2.feedback, 9);

    let mut m = mock2;
    m.control(|fbk| *fbk.0 = 11);

    let mock2 = m;
    assert_eq!(mock2.feedback, 11);
}
