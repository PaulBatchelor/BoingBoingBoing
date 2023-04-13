pub mod blep;
pub mod butterworth;
pub mod magic_circle;
pub mod bigverb;

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
