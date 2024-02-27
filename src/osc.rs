mod libs;

use crate::libs::notation::{fit_to_scale, gen_notes};
use autopilot::mouse::location;
use autopilot::screen::size;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SizedSample};
use libs::wav::BitDepth;

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");
    println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()).unwrap(),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()).unwrap(),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    };
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let mut ph = Phasor::new(440.0, config.sample_rate.0);
    let screen_size = size();
    let scale = gen_notes();
    let pentatonic = vec![
        scale[36 + 0],
        scale[36 + 3],
        scale[36 + 5],
        scale[36 + 6], // some blues
        scale[36 + 7],
        scale[36 + 10],
        scale[36 + 12],
    ];
    println!("{pentatonic:?}");
    let mut next_value = move || {
        let mouse_loc = location();
        // let nx = mouse_loc.x / screen_size.width;
        let ny = mouse_loc.y / screen_size.height;
        let raw_f = log_map(220.0, 440.0, (1.0 - ny as f32));
        let scale_index = fit_to_scale(&pentatonic, raw_f as f64);
        let f = pentatonic[scale_index] as f32;
        print!("y:{ny:.3} f:{f:.3}  \r");
        ph.set_f(f);

        triangle_osc(ph.next().unwrap())
    };

    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| write_data(data, channels, &mut next_value),
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis((100.0 * 1000.0) as u64));
    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn sine_osc(phase: f32) -> f32 {
    (phase * 2.0 * std::f32::consts::PI).sin()
}

fn square_osc(phase: f32) -> f32 {
    if phase > 0.5 {
        1.0
    } else {
        -1.0
    }
}

fn saw_osc(phase: f32) -> f32 {
    (2.0 * phase) - 1.0
}

fn triangle_osc(phase: f32) -> f32 {
    if phase < 0.5 {
        (4.0 * phase) - 1.0
    } else {
        (4.0 * (1.0 - phase)) - 1.0
    }
}

fn lin_map(min: f32, max: f32, v: f32) -> f32 {
    //for v in [0,1]
    min + (max - min) * v
}

fn log_map(min: f32, max: f32, v: f32) -> f32 {
    let semitones = 12.0 * (max / min).log2();
    min * 2.0f32.powf((v * semitones) / 12.0)
}

struct Phasor {
    f: f32,
    t: f32,
    sample_rate: u32,
}

impl Phasor {
    fn new(initial_f: f32, sample_rate: u32) -> Phasor {
        Phasor {
            f: initial_f,
            t: 0.0,
            sample_rate,
        }
    }
    fn set_f(&mut self, f: f32) {
        self.f = f;
    }
}

impl Iterator for Phasor {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        // Save current t to return
        let t = self.t;
        // Incremet t for next iteration
        self.t = (self.t + self.f / self.sample_rate as f32) % 1.0;
        Some(t)
    }
}
