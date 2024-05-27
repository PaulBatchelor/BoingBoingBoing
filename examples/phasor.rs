
use boingboingboing as boing;

const SAMPLERATE: usize = 44100;

fn mtof(nn: f32) -> f32 {
    let freq = (2.0_f32).powf((nn - 69.0) / 12.0) * 440.0;
    freq
}

fn main() {
    let mut ph1 = boing::phasor(SAMPLERATE, 0.);
    let mut ph2 = boing::phasor(SAMPLERATE, 0.);
    let mut butlp = boing::butlp(SAMPLERATE);
    let mut wav = boing::monowav("phasor.wav");

    ph1.set_freq(mtof(60.0));
    ph2.set_freq(1.0);
    butlp.set_freq(300.0);

    for _ in 0..(SAMPLERATE * 4) {
        // Modulate buzzing phasor 1 frequency with slow-moving phasor 2 
        let ramp = ph2.tick();
        ph1.set_freq(mtof(60.0 + 12.0*ramp));
        let sig = ph1.tick();

        // scale buzzer phasor to DC
        let sig = (2.0 * sig) - 1.0;
        let sig = sig * 0.8;
        let sig = butlp.tick(sig);

        // write to WAV
        wav.tick(sig)
    }
}
