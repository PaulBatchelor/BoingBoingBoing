pub mod blep;
pub mod butterworth;

pub fn blep(sr: usize) -> blep::BLEP {
    blep::BLEP::new(sr)
}

pub fn butlp(sr: usize) -> butterworth::ButterworthLowPass {
    butterworth::ButterworthLowPass::new(sr)
}
