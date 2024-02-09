mod libs;

use crate::libs::sampling;
use crate::libs::wav::{ BitDepth, WavFile, WavParams};


fn main() {
    let sample_rate = 44100;
    let bpm = 108.0;
    let whole_note = 60.0 / bpm * 2.0;
    let half_note = whole_note / 2.0;
    let quarter_note = half_note / 2.0;
    let eighth_note = quarter_note / 2.0;
    // let sixteenth_note = eighth_note / 2.0;
    let dot = 1.5;
    
    let note = 440.0;
    let semitone = 2.0f64.powf(1.0 / 12.0);
    let a4 = note;
    let e4 = a4 / semitone.powf(5.0);
    let c4 = a4 / semitone.powf(9.0);
    let g4 = a4 / semitone.powf(2.0);

    let volume = 0.5;

    let mut data = sampling::sine_wave_truncated(e4, sample_rate, half_note, BitDepth::U16(0), volume);
    data.append(&mut sampling::sine_wave_truncated(e4, sample_rate, half_note, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(e4, sample_rate, half_note, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(c4 , sample_rate, quarter_note*dot, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(g4 , sample_rate, eighth_note, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(e4, sample_rate, half_note, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(c4 , sample_rate, quarter_note*dot, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(g4 , sample_rate, eighth_note, BitDepth::U16(0), volume));
    data.append(&mut sampling::sine_wave_truncated(e4, sample_rate, whole_note, BitDepth::U16(0), volume));

    let mut octave = sampling::saw_wave_truncated(e4/2.0, sample_rate, half_note, BitDepth::U16(0), volume);
    octave.append(&mut sampling::saw_wave_truncated(e4/2.0, sample_rate, half_note, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(e4/2.0, sample_rate, half_note, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(c4/2.0 , sample_rate, quarter_note*dot, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(g4/2.0 , sample_rate, eighth_note, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(e4/2.0, sample_rate, half_note, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(c4/2.0 , sample_rate, quarter_note*dot, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(g4/2.0 , sample_rate, eighth_note, BitDepth::U16(0), volume));
    octave.append(&mut sampling::saw_wave_truncated(e4/2.0, sample_rate, whole_note, BitDepth::U16(0), volume));


    let mut output = Vec::new();
    let mut i = 0;
    for s in data {
        output.push(s + octave[i]);
        i += 1;
    };

    println!("Feel the evil");

    // print!("{:?}", data);
    let params = WavParams {
        sample_rate: sample_rate,
        channels: 1,
    };
    let wav = WavFile::new(params, output);
    wav.write("out/test.wav").unwrap();
    println!("Wrote test.wav");
}   

#[test]
fn test_main() {
    main();
}
