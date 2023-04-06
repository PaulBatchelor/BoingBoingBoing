pub mod blep;

pub fn blep(sr: usize) -> blep::BLEP {
    blep::BLEP::new(sr)
}
