use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Duration;
use reqwest::blocking::Client;
use hound::WavReader;

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
    let chunk_size = (spec.sample_rate as f32 * duration_ms / 1000.0) as usize * spec.channels as usize;

    let client = Client::new();
    let url = format!("http://localhost:{}/google/streaming", server_port);

    let mut buffer = vec![0u8; chunk_size * std::mem::size_of::<i16>()];
    let mut file = BufReader::new(File::open(wav_path)?);

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let response = client.post(&url)
            .body(buffer[..bytes_read].to_vec())
            .send()?;

        if !response.status().is_success() {
            eprintln!("Error: Server responded with status {}", response.status());
        }

        // Sleep for 100ms to simulate real-time streaming
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Streaming completed");
    Ok(())
}
