use crate::audio::{
    decode::derive_audio_key,
    resample::resample_audio,
    filter::filter_audio,
    normalize::normalize_audio,
};

use std::path::Path;

pub fn process_audio_key<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {

    // 1. Decode audio file
    let decoded = decode_audio_key(path)?;

    // 2. Resample to fixed rate
    let mut signal = resample_audio(&decoded);

    // 3. Remove unwanted frequencies (noise)
    filter_audio(
        &mut signal,
        16_000,
    );

    // 4. Normalize amplitude
    normalize_audio(&mut signal);


    Ok(signal)
}