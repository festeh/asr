use std::{fs::File, io::Read};
use base64;

pub fn read_file_bytes(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = vec![];
    // read file as byte stream
    file.read_to_end(&mut contents)?;
    // encode
    let contents = base64::encode(&contents);
    Ok(contents)
}
