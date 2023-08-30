use crate::signal::{const_, Sf64, Signal, Trigger};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Saw,
    Triangle,
    Pulse,
    Noise,
}

impl From<Waveform> for Signal<Waveform> {
    fn from(value: Waveform) -> Self {
        const_(value)
    }
}

pub struct Oscillator {
    pub waveform: Signal<Waveform>,
    pub frequency_hz: Sf64,
    pub pulse_width_01: Sf64,
    pub reset_trigger: Trigger,
    pub reset_offset_01: Sf64,
}

impl Oscillator {
    pub fn signal(mut self) -> Sf64 {
        let mut rng = StdRng::from_entropy();
        let mut state_opt = None;
        Signal::from_fn(move |ctx| {
            let state = match state_opt {
                None => self.reset_offset_01.sample(ctx),
                Some(state) => {
                    if self.reset_trigger.sample(ctx) {
                        self.reset_offset_01.sample(ctx)
                    } else {
                        state
                    }
                }
            };
            let state_delta = self.frequency_hz.sample(ctx) / ctx.sample_rate_hz;
            let state = (state + state_delta).rem_euclid(1.0);
            state_opt = Some(state);
            match self.waveform.sample(ctx) {
                Waveform::Sine => (state * PI * 2.0).sin(),
                Waveform::Saw => (state * 2.0) - 1.0,
                Waveform::Triangle => (((state * 2.0) - 1.0).abs() * 2.0) - 1.0,
                Waveform::Pulse => {
                    if state < self.pulse_width_01.sample(ctx) {
                        -1.0
                    } else {
                        1.0
                    }
                }
                Waveform::Noise => (rng.gen::<f64>() * 2.0) - 1.0,
            }
        })
    }
}
