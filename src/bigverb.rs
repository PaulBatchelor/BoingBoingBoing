// BigVerb
// Waveguide FDN Reverberator
// based on: https://pbat.ch/sndkit/bigverb/

use std::f32::consts::PI;

#[derive(Clone, Copy)]
struct ParamSet {
    delay: i32,
    drift: i32,
    randfreq: i32,
    seed: i32,
}

const PARAMS: [ParamSet; 8] = [
     ParamSet {delay:0x09a9, drift:0x0a, randfreq:0xc1c, seed:0x07ae},
     ParamSet {delay:0x0acf, drift:0x0b, randfreq:0xdac, seed:0x7333},
     ParamSet {delay:0x0c91, drift:0x11, randfreq:0x456, seed:0x5999},
     ParamSet {delay:0x0de5, drift:0x06, randfreq:0xf85, seed:0x2666},
     ParamSet {delay:0x0f43, drift:0x0a, randfreq:0x925, seed:0x50a3},
     ParamSet {delay:0x101f, drift:0x0b, randfreq:0x769, seed:0x5999},
     ParamSet {delay:0x085f, drift:0x11, randfreq:0x37b, seed:0x7333},
     ParamSet {delay:0x078d, drift:0x06, randfreq:0xc95, seed:0x3851},
];
const FRACSCALE: u32 = 0x10000000;
const FRACMASK: u32 = 0xFFFFFFF;
const FRACNBITS: u32 = 28;

#[derive(Clone, Copy)]
struct BigVerbDelay {
    bufstart: usize,
    sz: usize,
    wpos: i32,
    irpos: i32,
    frpos: i32,
    rng: i32,
    inc: i32,
    counter: i32,
    maxcount: i32,
    dels: f32,
    drift: f32,
    y: f32,
}

impl BigVerbDelay {
    pub fn new(bufstart: usize, sz: usize, p: &ParamSet, sr: usize) -> Self {
        let readpos = p.delay as f32 / 44100.0;
        let readpos =
            readpos +
            p.seed as f32 * (p.drift as f32 * 0.0001) / 32768.0; 
        let readpos = (sz as f32 - (readpos * sr as f32)).floor();
        BigVerbDelay {
            bufstart: bufstart,
            sz: sz,
            wpos: 0,
            irpos: readpos as i32,
            frpos: 0,
            rng: p.seed,
            inc: 0,
            counter: 0,
            maxcount: 0,
            dels: 0.0,
            drift: 0.0,
            y: 0.0,
        }
    }
}
