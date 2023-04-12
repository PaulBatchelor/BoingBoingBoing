#[derive(Clone, Copy)]
pub struct BLEP {
    freq: f32,
    pfreq: f32,
    onedsr: f32,
    inc: f32,
    phs: f32,
    prev: f32,
    r: f32,
    x: f32,
    y: f32,
}

fn polyblep(dt: f32, t: f32) -> f32 {

    if t < dt {
        let t = t / dt;
        return t + t - t * t - 1.0;
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        return t * t + t + t + 1.0;
    }

    0.0
}

impl BLEP {
    pub fn new(sr: usize) -> Self {
        BLEP {
            freq:  1000.0,
            pfreq: -1.0,
            onedsr: 1.0 / sr as f32,
            inc: 0.0,
            phs: 0.0,
            prev: 0.0,
            r: (-1.0 / (0.0025 * sr as f32)).exp(),
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn set_freq(&mut self, f: f32) {
        self.freq = f;
    }

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

    pub fn square(&mut self) -> f32 {
        if self.freq != self.pfreq {
            self.pfreq = self.freq;
            self.inc = self.freq * self.onedsr;
        }

        let phs = self.phs;

        let mut x;
        if phs < 0.5 {
            x = 1.0;
        } else {
            x = -1.0;
        }

        x += polyblep(self.inc, phs);
        x -= polyblep(self.inc, (phs + 0.5) % 1.0);
        let out = x;

        self.phs += self.inc;
        if self.phs > 1.0 {
            self.phs -= 1.0;
        }

        out
    }

    pub fn triangle(&mut self) -> f32 {
        if self.freq != self.pfreq {
            self.pfreq = self.freq;
            self.inc = self.freq * self.onedsr;
        }

        let phs = self.phs;

        let mut x;
        if phs < 0.5 {
            x = 1.0;
        } else {
            x = -1.0;
        }

        // compute square
        x += polyblep(self.inc, phs);
        x -= polyblep(self.inc, (phs + 0.5) % 1.0);

        // scale and integrate
        if self.freq != 0.0 {
            x *= 4.0 / self.freq; 
        } else {
            x = 0.0;
        }
        x += self.prev;
        self.prev = x;

        // dc blocker
        self.y = x - self.x + self.r*self.y;
        self.x = x;

        let out = self.y * 0.8;

        self.phs += self.inc;
        if self.phs > 1.0 {
            self.phs -= 1.0;
        }

        out
    }
}
