mod libs;
use libs::wav::BitDepth;

use crate::libs::amdf::amdf;
use crate::libs::notation::freq_to_note;
use crate::libs::wav::WavFile;
use std::borrow::Borrow;
use std::path::Path;

fn bit_depth_to_float(s: &BitDepth) -> f64 {
    match s {
        BitDepth::U8(v) => *v as f64,
        BitDepth::U16(v) => *v as f64,
        BitDepth::U32(v) => *v as f64,
    }
}

fn main() {
    // let file = WavFile::read(Path::new("samples/sine_pulse_440.wav")).unwrap();
    let file = WavFile::read(Path::new("out/test.wav")).unwrap();
    let sample_rate = file.hdr.fmt_ck.sample_rate;
    // file.write(Path::new("out/identity.wav")).unwrap();
    let step = (sample_rate as usize / 20); // 20 hz as minimal detection
    for i in (0..(file.data.len() - step)).step_by(step) {
        let samples = file.data[i..(i + step)]
            .iter()
            .map(bit_depth_to_float)
            .collect();
        let wave_period = amdf(samples);
        let freq = sample_rate as f64 / wave_period as f64;
        println!("{}", freq_to_note(freq));
    }
}
