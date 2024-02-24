pub mod io;
pub mod resample;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::platform::AlsaDevice;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::Device;
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
    println!("Channels: {}", config.channels());
    println!("Sample rate: {}", config.sample_rate().0);
    println!("Sample format: {:?}", config.sample_format());
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

pub struct AudioRecorder {
    stop_signal: Arc<Mutex<bool>>,
    config: cpal::SupportedStreamConfig,
    device: cpal::Device,
    path: Arc<Mutex<Option<PathBuf>>>,
}

impl Default for AudioRecorder {
    fn default() -> Self {
        let host = cpal::default_host();
        let mut device = host.default_input_device().unwrap();
        host.input_devices().unwrap().for_each(|indevice| {
            println!("Device: {:?}", indevice.name().unwrap());
            if indevice.name().unwrap().contains("front:CARD=K66") {
                println!("Setting device to: {:?}", indevice.name().unwrap());
                device = indevice;
                return;
            }
            println!();
        });
        let config = device.default_input_config().unwrap();
        println!("Default input config: {:?}", config);

        Self {
            config,
            device,
            path: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(Mutex::new(false)),
        }
    }
}

impl AudioRecorder {
    pub fn get_path(&self) -> Option<PathBuf> {
        self.path.lock().unwrap().clone()
    }

    pub fn set_path(&self) {
        let root = dirs::data_dir().unwrap();
        let audio_dir = root.join("audios");
        std::fs::create_dir_all(&audio_dir).unwrap();
        let timestamp = chrono::Local::now().format("%H-%M-%S-%d-%m-%Y").to_string();
        let filneame = format!("{}.wav", timestamp);
        *self.path.lock().unwrap() = Some(audio_dir.join(filneame));
    }

    pub fn get_tmp_path(&self) -> PathBuf {
        let root = dirs::data_dir().unwrap();
        let audio_dir = root.join("audios");
        std::fs::create_dir_all(&audio_dir).unwrap();
        let filneame = "tmp.wav";
        audio_dir.join(filneame)
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let spec = wav_spec_from_config(&self.config);
        let tmp_path = self.get_tmp_path();
        if let Ok(mut file) = File::create(&tmp_path) {
            file.write_all(&[])?;
        }
        let writer = hound::WavWriter::create(tmp_path.clone(), spec).unwrap();
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
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )))
            }
        };
        *self.stop_signal.lock().unwrap() = false;
        println!("Recording to {:?}.", tmp_path);
        stream.play()?;
        let mut max_duration = std::time::Duration::from_secs(60);
        while !*self.stop_signal.lock().unwrap() {
            std::thread::sleep(std::time::Duration::from_millis(100));
            max_duration -= std::time::Duration::from_millis(100);
            if max_duration.as_secs() == 0 {
                break;
            }
            println!("Recording...");
        }
        drop(stream);
        writer.lock().unwrap().take().unwrap().finalize()?;
        println!("Recording complete.");
        *self.stop_signal.lock().unwrap() = false;
        Ok(())
    }

    pub fn order_stop(&self) {
        println!("Set should_stop to true.");
        *self.stop_signal.lock().unwrap() = true;
    }

    pub fn is_stopping(&self) -> bool {
        *self.stop_signal.lock().unwrap()
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
