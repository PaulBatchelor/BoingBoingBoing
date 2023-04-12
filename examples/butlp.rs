// Produces raw floating point samples and writes them to disk
// convert to WAV using SoX:
// sox -t raw -r 44100 -e floating-point -c 1 -b 32 test.bin test.wav

use boingboingboing as boing;
use std::fs::File;
use std::io::prelude::*;

const SAMPLERATE: usize = 44100;

fn main() {
    let mut blsaw = boing::blep(SAMPLERATE);
    let mut butlp = boing::butlp(SAMPLERATE);

    let mut blk: [f32; 64] = [0.0; 64];
    let mut bytes: [u8; 256] = [0; 256];
    let nblks = (44100 * 5) / 64;
    let file = File::create("test.bin");

    blsaw.set_freq(440.0);
    butlp.set_freq(300.0);

    for _ in 0..nblks {
        for n in 0..64 {
            let smp = blsaw.saw();
            let smp = butlp.tick(smp);
            blk[n] = smp * 0.3;
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
