pub struct BLEP {
    freq: f32,
    pfreq: f32,
    onedsr: f32,
    inc: f32,
    phs: f32,
    //a: f32,
    //prev: f32,
    //r: f32,
    //x: f32,
    //y: f32,
}

fn polyblep(dt: f32, t: f32) -> f32 {

    if t < dt {
        let t = t / dt;
        return t + t - t * t - 1.0;
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        return t * t + t + t + 1.0;
    }

    return 0.0;
}

impl BLEP {
    pub fn new(sr: usize) -> Self {
        let blep = BLEP {
            freq:  1000.0,
            pfreq: -1.0,
            onedsr: 1.0 / sr as f32,
            inc: 0.0,
            phs: 0.0,
            // a: (-1.0 / (0.1 * sr as f32)).exp(),
            // prev: 0.0,
            // r: (-1.0 / (0.0025 * sr as f32)).exp(),
            // x: 0.0,
            // y: 0.0,
        };

        blep
    }

    pub fn set_freq(&mut self, f: f32) {
        self.freq = f;
    }

// static SKFLT blep_saw(sk_blep *blep, SKFLT t)
// {
//     SKFLT value;
// 
//     value = (2.0 * t) - 1.0;
//     value -= polyblep(blep->inc, t);
// 
//     return value;
// }
//
//     SKFLT out;
// 
//     out = 0.0;
// 
// if (blep->freq != blep->pfreq) {
//     blep->pfreq = blep->freq;
//     blep->inc = blep->freq * blep->onedsr;
// }
// out = wave(blep, blep->phs);
// blep->phs += blep->inc;
// 
// if (blep->phs > 1.0) {
//     blep->phs -= 1.0;
// }

//

    pub fn saw(&mut self) -> f32 {
        let mut x;
        if self.freq != self.pfreq {
            self.pfreq = self.freq;
            self.inc = self.freq * self.onedsr;
        }

        let phs = self.phs;
        x = (2.0  * phs) - 1.0;
        x -= polyblep(self.inc, phs);
        let out = x;

        self.phs += self.inc;
        if self.phs > 1.0 {
            self.phs -= 1.0;
        }

        out
    }
}
