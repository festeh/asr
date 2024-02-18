use app::resample::{resample_audio, write_resampled};

fn main() {
    let path = "/tmp/recording.wav";
    println!("Resampling {}", path);
    let (left, right) = resample_audio(path).unwrap();
    write_resampled(left, right, "/tmp/resampled.wav");
}
