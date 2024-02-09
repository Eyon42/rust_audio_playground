use crate::libs::wav::BitDepth;

pub fn sine_wave(freq: f64, sample_rate: u32, duration: f64, bit_depth: BitDepth, volume: f64) -> Vec<BitDepth> {
    let num_samples = (duration * sample_rate as f64) as usize;
    let mut samples: Vec<BitDepth> = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let sample_max = match bit_depth {
            BitDepth::U8(_) => i8::MAX as f64,
            BitDepth::U16(_) => i16::MAX as f64,
            BitDepth::U32(_) => i32::MAX as f64,
        } * volume;
        let t = ((i as f64 / sample_rate as f64) * freq * 2.0 * std::f64::consts::PI).sin() * sample_max;
        samples.push(match bit_depth {
          BitDepth::U8(_) => BitDepth::U8(t as i8),
          BitDepth::U16(_) => BitDepth::U16(t as i16),
          BitDepth::U32(_) => BitDepth::U32(t as i32),
    });
    }
    samples
}

// TODO build sample builder


pub fn sine_wave_truncated(freq: f64, sample_rate: u32, duration: f64, bit_depth: BitDepth, volume: f64) -> Vec<BitDepth> {
    let num_samples = (duration * sample_rate as f64) as usize;
    let mut samples: Vec<BitDepth> = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let sample_max = match bit_depth {
            BitDepth::U8(_) => i8::MAX as f64,
            BitDepth::U16(_) => i16::MAX as f64,
            BitDepth::U32(_) => i32::MAX as f64,
        } * volume;
        let v = sample_max * (num_samples as f64 - i as f64) / num_samples as f64;
        let t = ((i as f64 / sample_rate as f64)
            * freq * 2.0 * std::f64::consts::PI).sin()
            * v;
        samples.push(match bit_depth {
          BitDepth::U8(_) => BitDepth::U8(t as i8),
          BitDepth::U16(_) => BitDepth::U16(t as i16),
          BitDepth::U32(_) => BitDepth::U32(t as i32),
    });
    }
    samples
}

pub fn saw_wave_truncated(freq: f64, sample_rate: u32, duration: f64, bit_depth: BitDepth, volume: f64) -> Vec<BitDepth> {
    let num_samples = (duration * sample_rate as f64) as usize;
    let mut samples: Vec<BitDepth> = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let sample_max = match bit_depth {
            BitDepth::U8(_) => i8::MAX as f64,
            BitDepth::U16(_) => i16::MAX as f64,
            BitDepth::U32(_) => i32::MAX as f64,
        } * volume;
        let v = sample_max * (num_samples as f64 - i as f64) / num_samples as f64;
        let t = (1.0f64 / freq) * sample_rate as f64;
        let t = ((i as f64 % t)/t -0.5) *v;
        samples.push(match bit_depth {
          BitDepth::U8(_) => BitDepth::U8(t as i8),
          BitDepth::U16(_) => BitDepth::U16(t as i16),
          BitDepth::U32(_) => BitDepth::U32(t as i32),
    });
    }
    samples
}