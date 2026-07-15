use crate::audio::{
    decode::decode_audio_key,
    resample::resample_audio,
};

use std::path::Path;

pub fn process_audio_key<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {

    // Decode audio file
    let decoded = decode_audio_key(path)?;

    // Resample to fixed rate
    let signal = resample_audio(&decoded);

    Ok(signal)
}