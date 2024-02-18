pub mod io;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::{FromSample, Sample};

#[derive(Clone)]
pub struct Audio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            samples: vec![],
            sample_rate: 16000,
        }
    }
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        println!("Sample format is float");
        hound::SampleFormat::Float
    } else {
        println!("Sample format is int");
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

pub struct AudioRecorder {
    should_stop: Arc<Mutex<bool>>,
    config: cpal::SupportedStreamConfig,
    device: cpal::Device,
    pub path: String,
}

impl Default for AudioRecorder {
    fn default() -> Self {
        let path = "/tmp/recording.wav";
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let config = device.default_input_config().unwrap();

        let path = path.to_string();
        Self {
            config,
            device,
            path,
            should_stop: Arc::new(Mutex::new(false)),
        }
    }
}

impl AudioRecorder {
    pub fn start(&self) -> anyhow::Result<()> {
        let spec = wav_spec_from_config(&self.config);
        if let Ok(mut file) = File::create(&self.path) {
            file.write_all(&[])?;
        }
        let writer = hound::WavWriter::create(self.path.clone(), spec).unwrap();
        let writer = Arc::new(Mutex::new(Some(writer)));
        let writer_2 = writer.clone();
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )))
            }
        };
        println!("Recording to {:?}.", self.path);
        stream.play()?;
        while !*self.should_stop.lock().unwrap() {
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("Recording...");
        }
        drop(stream);
        writer.lock().unwrap().take().unwrap().finalize()?;
        println!("Recording complete.");
        *self.should_stop.lock().unwrap() = false;
        Ok(())
    }

    pub fn stop(&self) {
        *self.should_stop.lock().unwrap() = true;
    }

    pub fn is_stopped(&self) -> bool {
        *self.should_stop.lock().unwrap()
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
