// "magic circle" sinusoid generator
// https://ccrma.stanford.edu/~jos/pasp/Digital_Sinusoid_Generators.html

use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct MagicCircleSine {
    freq: f32,
    pfreq: f32,
    x1: f32,
    x2: f32,
    eps: f32,
    sr: usize,
}

impl MagicCircleSine {
    pub fn new(sr: usize) -> Self {
        MagicCircleSine {
            freq: 1000.0,
            pfreq: -1.0,
            x1: 1.0,
            x2: 0.0,
            eps: 0.0,
            sr: sr,
        }
    }

    pub fn set_freq(&mut self, f: f32) {
        self.freq = f;
    }

    pub fn tick(&mut self) -> f32 {
        if self.pfreq != self.freq {
            self.pfreq = self.freq;
            self.eps = 2.0 *
                (PI * (self.freq / self.sr as f32)).sin();
        }

        self.x1 = self.x1 + self.eps*self.x2;
        self.x2 = -self.eps*self.x1 + self.x2;

        self.x2
    }
}
