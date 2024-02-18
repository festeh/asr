use std::io::prelude::{Read, Seek, Write};

// resample a file with rubato
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

pub fn resample_audio(path: &str) -> Result<(Vec<f32>, Vec<f32>), Box<dyn std::error::Error>> {
    println!("Reading {}", path);
    let mut reader = hound::WavReader::open(path).unwrap();

    // Get the samples from the WAV file
    let samples: Vec<f32> = reader.samples::<f32>().map(|s| s.unwrap()).collect();
    println!("Read {} samples", samples.len());
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (i, sample) in samples.into_iter().enumerate() {
        if i % 2 == 0 {
            left.push(sample);
        } else {
            right.push(sample);
        }
    }

    // Create a resampler
    let sinc_params = SincInterpolationParameters {
        sinc_len: 256,
        window: WindowFunction::BlackmanHarris2,
        oversampling_factor: 160,
        interpolation: SincInterpolationType::Linear,
        f_cutoff: 0.95,
    };
    let mut resampler =
        SincFixedIn::<f32>::new(16000.0 / 44100.0, 1.0, sinc_params, left.len(), 1)?;

    // Resample the audio
    let resampled_left = resampler.process(&[left], None).unwrap();
    let resampled_right = resampler.process(&[right], None).unwrap();

    println!("Number of channels: {}", resampled_left.len());
    println!("Resampled to {} samples", resampled_left[0].len());
    let resampled_left = resampled_left[0].clone();
    let resampled_right = resampled_right[0].clone();
    Ok((resampled_left, resampled_right))
}

pub fn write_resampled(left: Vec<f32>, right: Vec<f32>, output_path: &str) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(output_path, spec).unwrap();

    for (left_sample, right_sample) in left.iter().zip(right.iter()) {
        let left_amplitude = (left_sample * i16::MAX as f32) as i16;
        let right_amplitude = (right_sample * i16::MAX as f32) as i16;
        writer.write_sample(left_amplitude).unwrap();
        writer.write_sample(right_amplitude).unwrap();
    }

    println!("Wrote resampled audio to {}", output_path);
    writer.finalize().unwrap();
}
