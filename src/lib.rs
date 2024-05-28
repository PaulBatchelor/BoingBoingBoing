pub mod bigverb;
pub mod blep;
pub mod butterworth;
pub mod magic_circle;
pub mod monowav;
pub mod phasor;
use std::fs::File;

pub fn blep(sr: usize) -> blep::BLEP {
    blep::BLEP::new(sr)
}

pub fn butlp(sr: usize) -> butterworth::ButterworthLowPass {
    butterworth::ButterworthLowPass::new(sr)
}

pub fn mcsine(sr: usize) -> magic_circle::MagicCircleSine {
    magic_circle::MagicCircleSine::new(sr)
}

pub fn bigverb(sr: usize) -> bigverb::BigVerb {
    bigverb::BigVerb::new(sr)
}

pub fn monowav(wavfilename: &str) -> monowav::MonoWav {
    let wav = File::create(wavfilename).unwrap();
    monowav::MonoWav::new(wav)
}

pub fn phasor(sr: usize, iphs: f32) -> phasor::Phasor {
    phasor::Phasor::new(sr, iphs)
}

pub fn mtof(nn: f32) -> f32 {
    let freq = (2.0_f32).powf((nn - 69.0) / 12.0) * 440.0;
    freq
}
