mod libs;
use crate::libs::wav::WavFile;
use std::path::Path;

fn main() {
    let file = WavFile::read(Path::new("samples/sine_pulse_440.wav")).unwrap();
    file.write(Path::new("out/identity.wav")).unwrap();
}
