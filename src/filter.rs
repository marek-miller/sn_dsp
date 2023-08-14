use crate::{
    frame::Frame,
    node::Node,
    num::{
        one,
        zero,
        Float,
        Fp,
        Real,
    },
};

#[derive(Debug, Clone)]
pub struct OnePole<T>
where
    T: Frame,
{
    b0: T::Sample,
    a1: T::Sample,
    y1: T,
}

impl<T> OnePole<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            b0: one(),
            a1: zero(),
            y1: zero(),
        }
    }
}

impl<T> Default for OnePole<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Frame> Node for OnePole<T> {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = *frm * self.b0 - self.y1 * self.a1;
            self.y1 = y0;
            *frm = y0;
        }
    }
}

/// DC Blocking filter
#[derive(Debug)]
pub struct DCBlock<T> {
    pub cutoff: Fp,
    x1:         T,
    y1:         T,
}

impl<T> DCBlock<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new(cutoff: Fp) -> Self {
        Self {
            cutoff,
            x1: zero(),
            y1: zero(),
        }
    }
}

impl<T> From<Fp> for DCBlock<T>
where
    T: Frame,
{
    fn from(value: Fp) -> Self {
        Self::new(value)
    }
}

impl<T> Node for DCBlock<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = *frm - self.x1 + self.y1 * (1.0 - self.cutoff).to_float();
            self.x1 = *frm;
            self.y1 = y0;
            *frm = y0;
        }
    }
}

// Biquad filter from [`FunDSP`](https://github.com/SamiPerttu/fundsp)
// by Sami Perttu. `FunDSP` is licensed under MIT License.
#[derive(Debug, Copy, Clone)]
pub struct BiquadCoefs {
    pub a1: Fp,
    pub a2: Fp,
    pub b0: Fp,
    pub b1: Fp,
    pub b2: Fp,
}

impl BiquadCoefs {
    /// Arbitrary biquad.
    #[must_use]
    pub fn arbitrary(
        a1: Fp,
        a2: Fp,
        b0: Fp,
        b1: Fp,
        b2: Fp,
    ) -> Self {
        Self {
            a1,
            a2,
            b0,
            b1,
            b2,
        }
    }

    /// Returns settings for a Butterworth lowpass filter.
    /// Cutoff is the -3 dB point of the filter.
    #[must_use]
    pub fn butter_lowpass(cutoff: Fp) -> Self {
        let f: Fp = <Fp as Real>::tan(cutoff * <Fp as Real>::PI);
        let a0r: Fp = 1.0 / (1.0 + <Fp as Real>::SQRT_2 * f + f * f);
        let a1: Fp = (2.0 * f * f - 2.0) * a0r;
        let a2: Fp = (1.0 - <Fp as Real>::SQRT_2 * f + f * f) * a0r;
        let b0: Fp = f * f * a0r;
        let b1: Fp = 2.0 * b0;
        let b2: Fp = b0;
        Self {
            a1,
            a2,
            b0,
            b1,
            b2,
        }
    }

    /// Returns settings for a Butterworth hipass filter.
    /// Cutoff is the -3 dB point of the filter.
    #[must_use]
    pub fn butter_hipass(cutoff: Fp) -> Self {
        let f: Fp = <Fp as Real>::tan(cutoff * <Fp as Real>::PI);
        let a0r: Fp = 1.0 / (1.0 + <Fp as Real>::SQRT_2 * f + f * f);
        let a1: Fp = (2.0 * f * f - 2.0) * a0r;
        let a2: Fp = (1.0 - <Fp as Real>::SQRT_2 * f + f * f) * a0r;
        let b0: Fp = 1. - f * f * a0r;
        let b1: Fp = -2.0 * b0;
        let b2: Fp = b0;
        Self {
            a1,
            a2,
            b0,
            b1,
            b2,
        }
    }

    /// Returns settings for a constant-gain bandpass resonator.
    /// The overall gain of the filter is independent of bandwidth.
    #[must_use]
    pub fn resonator(
        center: Fp,
        bandwidth: Fp,
    ) -> Self {
        let r: Fp = <Fp as Real>::exp(-<Fp as Real>::PI * bandwidth);
        let a1: Fp = -2.0 * r * <Fp as Real>::cos(<Fp as Real>::TAU * center);
        let a2: Fp = r * r;
        let b0: Fp = <Fp as Real>::sqrt(1.0 - r * r) * 0.5;
        let b1: Fp = 0.0;
        let b2: Fp = -b0;
        Self {
            a1,
            a2,
            b0,
            b1,
            b2,
        }
    }
}

impl Default for BiquadCoefs {
    fn default() -> Self {
        Self {
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

/// 2nd order IIR filter implemented in normalized Direct form I.
#[derive(Debug)]
pub struct Biquad<T> {
    pub coefs: BiquadCoefs,
    x1:        T,
    x2:        T,
    y1:        T,
    y2:        T,
}

impl<T> Biquad<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new(coefs: BiquadCoefs) -> Self {
        Self {
            coefs,
            x1: zero(),
            x2: zero(),
            y1: zero(),
            y2: zero(),
        }
    }

    pub fn coeffs(&self) -> &BiquadCoefs {
        &self.coefs
    }

    pub fn coeffs_mut(&mut self) -> &mut BiquadCoefs {
        &mut self.coefs
    }
}

impl<T> Default for Biquad<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new(BiquadCoefs::default())
    }
}

impl<T> From<BiquadCoefs> for Biquad<T>
where
    T: Frame,
{
    fn from(value: BiquadCoefs) -> Self {
        Self::new(value)
    }
}

impl<T> Node for Biquad<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let b0 = self.coefs.b0.to_float();
            let b1 = self.coefs.b1.to_float();
            let b2 = self.coefs.b2.to_float();
            let a1 = self.coefs.a1.to_float();
            let a2 = self.coefs.a2.to_float();

            let x0 = *frm;
            let y0 = x0 * b0 + self.x1 * b1 + self.x2 * b2
                - self.y1 * a1
                - self.y2 * a2;
            self.x2 = self.x1;
            self.x1 = x0;
            self.y2 = self.y1;
            self.y1 = y0;
            *frm = y0;
        }
    }
}

#[must_use]
pub fn butter_lowpass<T: Frame>(cutoff: Fp) -> Biquad<T> {
    Biquad::new(BiquadCoefs::butter_lowpass(cutoff))
}

#[must_use]
pub fn butter_hipass<T: Frame>(cutoff: Fp) -> Biquad<T> {
    Biquad::new(BiquadCoefs::butter_hipass(cutoff))
}

#[must_use]
pub fn resonator<T: Frame>(
    center: Fp,
    bandwidth: Fp,
) -> Biquad<T> {
    Biquad::new(BiquadCoefs::resonator(center, bandwidth))
}

/// Nth order Butterworth lowpass filter
#[derive(Debug)]
pub struct Lpf<const N: usize, T> {
    filters: [Biquad<T>; N],
}

impl<const N: usize, T> Lpf<N, T>
where
    T: Frame,
{
    #[must_use]
    pub fn new(cutoff: Fp) -> Self {
        Self {
            filters: [0; N].map(|_| butter_lowpass(cutoff)),
        }
    }
}

impl<const N: usize, T> From<Fp> for Lpf<N, T>
where
    T: Frame,
{
    fn from(value: Fp) -> Self {
        Self::new(value)
    }
}

impl<const N: usize, T> Node for Lpf<N, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        self.filters
            .as_mut_slice()
            .iter_mut()
            .for_each(|f| f.proc(frames));
    }
}

#[must_use]
pub fn lpf<const N: usize, T: Frame>(cutoff: Fp) -> Lpf<N, T> {
    Lpf::new(cutoff)
}

#[must_use]
pub fn lpf2<T: Frame>(cutoff: Fp) -> Lpf<2, T> {
    Lpf::new(cutoff)
}

#[must_use]
pub fn lpf4<T: Frame>(cutoff: Fp) -> Lpf<4, T> {
    Lpf::new(cutoff)
}

#[must_use]
pub fn lpf8<T: Frame>(cutoff: Fp) -> Lpf<8, T> {
    Lpf::new(cutoff)
}
