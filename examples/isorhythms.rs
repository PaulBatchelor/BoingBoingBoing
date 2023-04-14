// Produces raw floating point samples and writes them to disk
// convert to WAV using SoX:
// sox -t raw -r 44100 -e floating-point -c 1 -b 32 test.bin test.wav

use boingboingboing as boing;
use std::fs::File;
use std::io::prelude::*;

const SAMPLERATE: usize = 44100;

fn mtof(nn: f32) -> f32 {
    let freq = (2.0_f32).powf((nn - 69.0) / 12.0) * 440.0;
    freq
}

#[derive(Clone, Copy)]
pub struct Voice {
    blsaw: boing::blep::BLEP,
    lpf: boing::butterworth::ButterworthLowPass,
    lfo: boing::magic_circle::MagicCircleSine,
    phs: f32,
}

impl Voice {
    pub fn new(sr: usize) -> Self {
        Voice {
            blsaw: boing::blep(sr),
            lpf: boing::butlp(sr),
            lfo: boing::mcsine(sr),
            phs: 0.0,
        }
    }

    pub fn pitch(&mut self, nn: f32) {
        self.blsaw.set_freq(mtof(nn));
    }

    pub fn rate(&mut self, freq: f32) {
        self.lfo.set_freq(freq);
    }

    pub fn phase(&mut self, phs: f32) {
        self.phs = phs;
    }

    pub fn init(&mut self) {
        self.pitch(60.0);
        self.rate(0.3);
    }

    pub fn tick(&mut self) -> f32 {
        let smp = self.blsaw.saw();
        let s = (1.0 + self.lfo.tick()) * 0.5;
        self.lpf.set_freq(100.0 + 400.0 * s);
        let smp = self.lpf.tick(smp);
        return smp * 0.3 * s;
    }
}

fn main() {
    let base = 60.0 - 4.0;
    let mut voices: [Voice; 6] = [Voice::new(SAMPLERATE); 6];
    let mut bigverb = boing::bigverb(SAMPLERATE);
    bigverb.init();

    let mut blk: [f32; 64] = [0.0; 64];
    let mut bytes: [u8; 256] = [0; 256];
    let nblks = (44100 * 60) / 64;
    let file = File::create("test.bin");

    voices[0].init();
    voices[0].pitch(base + 12.0);
    voices[0].rate(1.0 / 10.0);
    voices[0].phase(0.0);

    voices[1].init();
    voices[1].pitch(base + 11.0);
    voices[1].rate(1.0 / 9.0);
    voices[1].phase(0.1);

    voices[2].init();
    voices[2].pitch(base + 7.0);
    voices[2].rate(1.0 / 7.0);
    voices[2].phase(0.2);

    voices[3].init();
    voices[3].pitch(base);
    voices[3].rate(1.0 / 6.0);
    voices[3].phase(0.3);

    voices[4].init();
    voices[4].pitch(base + 2.0);
    voices[4].rate(1.0 / 5.0);

    voices[5].init();
    voices[5].pitch(base + 4.0);
    voices[5].rate(1.0 / 4.0);

    for _ in 0..nblks {
        for n in 0..64 {
            let mut out = 0.0;
            for v in 0..6 {
                out += voices[v].tick();
            }
            let (revl, _) = bigverb.tick(out, out);
            blk[n] = out + revl * 0.2;
        }

        for n in 0..64 {
            let pos = n * 4;
            let b = blk[n].to_le_bytes();
            bytes[pos] = b[0];
            bytes[pos + 1] = b[1];
            bytes[pos + 2] = b[2];
            bytes[pos + 3] = b[3];
        }

        _ = file.as_ref().unwrap().write_all(&bytes);
    }
}
