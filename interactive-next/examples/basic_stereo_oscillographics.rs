use caw_builders::*;
use caw_core_next::*;
use caw_interactive_next::{
    input::Input,
    window::{Visualization, Window},
};
use rgb_int::Rgb24;

fn signal_left(input: Input) -> Sig<impl SigT<Item = f32>> {
    let freq = Sig(30.0)
        + (input.mouse.y_01() * Sig(500.0))
        + (oscillator(waveform::Triangle, 0.002)
            .reset_offset_01(0.25)
            .build())
            * Sig(1.0);
    oscillator(waveform::Sine, freq).build()
}

fn signal_right(input: Input) -> Sig<impl SigT<Item = f32>> {
    let freq = Sig(30.0)
        + (input.mouse.x_01() * Sig(500.0))
        + (oscillator(waveform::Triangle, -0.0013)
            .reset_offset_01(0.25)
            .build())
            * Sig(1.0);
    oscillator(waveform::Sine, freq).build()
}

fn run() -> anyhow::Result<()> {
    let window = Window::builder()
        .scale(1.0)
        .stable(false)
        .line_width(1)
        .background(Rgb24::new(0, 0, 0))
        .foreground(Rgb24::new(0, 255, 0))
        .visualization(Visualization::StereoOscillographics)
        .build();
    let input = window.input();
    window.play_stereo(signal_left(input.clone()), signal_right(input.clone()))
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    run()
}