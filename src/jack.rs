mod libs;

use crate::libs::wav::WavFile;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SizedSample};
use libs::wav::BitDepth;
use std::path::Path;

fn bit_depth_to_float(s: &BitDepth) -> f32 {
    let v = match s {
        BitDepth::U8(v) => *v as f32 / i8::MAX as f32,
        BitDepth::U16(v) => *v as f32 / i16::MAX as f32,
        BitDepth::U32(v) => *v as f32 / i32::MAX as f32,
    };
    v
}

fn main() {
    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
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
    let file = WavFile::read(Path::new("out/test.wav")).unwrap();

    let file_data: Vec<f32> = file.data.iter().map(bit_depth_to_float).collect();
    let audio_length = file_data.len() as f32 / file.hdr.fmt_ck.sample_rate as f32;

    let mut s: usize = 0;
    let mut next_value = move || {
        let sample_data = file_data.get(s);
        match sample_data {
            Some(sample) => {
                s += 1;
                *sample
            }
            _ => 0.0, // Pad with zeroes after it ends
        }
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
    std::thread::sleep(std::time::Duration::from_millis(
        (audio_length * 1000.0) as u64,
    ));
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
