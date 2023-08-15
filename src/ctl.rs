pub trait Reset {
    fn reset(&mut self);
}

pub trait Control {
    type Ctl<'a>
    where
        Self: 'a;

    fn control(
        &mut self,
        f: impl FnOnce(&mut Self::Ctl<'_>),
    );
}

impl<T> Reset for T
where
    T: Control,
    for<'a> <T as Control>::Ctl<'a>: Reset,
{
    fn reset(&mut self) {
        self.control(|ctl| ctl.reset());
    }
}
