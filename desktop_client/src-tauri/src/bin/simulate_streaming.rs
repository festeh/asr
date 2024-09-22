use std::env;
use std::time::Duration;
use reqwest::blocking::Client;
use hound::WavReader;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <path_to_wav_file> <server_port>", args[0]);
        std::process::exit(1);
    }

    let wav_path = &args[1];
    let server_port = &args[2];

    let mut reader = WavReader::open(wav_path)?;
    let spec = reader.spec();
    let duration_ms = 100.0;
    let samples_per_chunk = (spec.sample_rate as f32 * duration_ms / 1000.0) as usize;
    let bytes_per_sample = (spec.bits_per_sample / 8) as usize;
    let chunk_size = samples_per_chunk * spec.channels as usize * bytes_per_sample;

    let client = Client::new();
    let url = format!("http://localhost:{}/google/streaming", server_port);

    let samples: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();
    let mut chunks = samples.chunks(samples_per_chunk * spec.channels as usize);

    while let Some(chunk) = chunks.next() {
        let mut buffer = Vec::with_capacity(chunk_size);
        for &sample in chunk {
            buffer.extend_from_slice(&sample.to_le_bytes());
        }

        let response = client.post(&url)
            .body(buffer)
            .send()?;

        if !response.status().is_success() {
            eprintln!("Error: Server responded with status {}", response.status());
        }

        // Print the response content
        let response_text = response.text()?;
        println!("Server response: {}", response_text);

        // Sleep for 100ms to simulate real-time streaming
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Streaming completed");
    Ok(())
}
