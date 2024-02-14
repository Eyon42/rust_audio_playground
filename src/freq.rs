mod libs;
use libs::wav::BitDepth;

use crate::libs::amdf::amdf;
use crate::libs::wav::WavFile;
use std::path::Path;

fn bit_depth_to_float(s: &BitDepth) -> f64 {
    match s {
        BitDepth::U8(v) => *v as f64,
        BitDepth::U16(v) => *v as f64,
        BitDepth::U32(v) => *v as f64,
    }
}

fn main() {
    let file = WavFile::read(Path::new("samples/sine_pulse_440.wav")).unwrap();
    // file.write(Path::new("out/identity.wav")).unwrap();
    let mut samples = file.data.iter().map(bit_depth_to_float).collect();
    amdf()
}
